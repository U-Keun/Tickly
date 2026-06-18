import CloudKit
import Darwin
import Foundation

private let ticklyICloudContainerIdentifier = "iCloud.com.u-keunsong.tickly"
private let ticklyICloudZoneName = "TicklyV2"
private let ticklyICloudRecordTypes = [
    "TicklyV2Category",
    "TicklyV2Tag",
    "TicklyV2Todo",
    "TicklyV2TodoTag",
    "TicklyV2CompletionLog"
]

private func ticklyICloudUnavailableJSON(status: String, error: String) -> String {
    ticklyEncodeJSONObject([
        "available": false,
        "status": status,
        "error": error,
        "records": [[String: Any]](),
        "synced_at": NSNull()
    ])
}

@_cdecl("tickly_perform_icloud_sync")
public func tickly_perform_icloud_sync(
    _ webViewPointer: UnsafeMutableRawPointer?,
    _ viewControllerPointer: UnsafeMutableRawPointer?,
    _ requestPointer: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
    guard let requestPointer else {
        return strdup(ticklyICloudUnavailableJSON(
            status: "error",
            error: "Missing iCloud sync request."
        ))
    }

    let requestJSON = String(cString: requestPointer)
    var resultJSON = ""
    let semaphore = DispatchSemaphore(value: 0)

    if #available(iOS 17.0, *) {
        Task {
            resultJSON = await TicklyICloudSync.perform(requestJSON: requestJSON)
            semaphore.signal()
        }
    } else {
        resultJSON = ticklyICloudUnavailableJSON(
            status: "unsupported_os",
            error: "iCloud sync requires iOS 17 or later."
        )
        semaphore.signal()
    }

    if semaphore.wait(timeout: .now() + 40) == .timedOut {
        resultJSON = ticklyICloudUnavailableJSON(
            status: "timeout",
            error: "iCloud sync timed out."
        )
    }

    return strdup(resultJSON)
}

@_cdecl("tickly_free_c_string")
public func tickly_free_c_string(_ pointer: UnsafeMutablePointer<CChar>?) {
    free(pointer)
}

@available(iOS 17.0, *)
private enum TicklyICloudSync {
    static func perform(requestJSON: String) async -> String {
        do {
            guard
                let requestData = requestJSON.data(using: .utf8),
                let request = try JSONSerialization.jsonObject(with: requestData) as? [String: Any]
            else {
                return unavailableJSON(status: "error", error: "Invalid iCloud sync request.")
            }

            let action = request["action"] as? String ?? "availability"
            let container = CKContainer(identifier: ticklyICloudContainerIdentifier)
            let accountStatus = try await container.accountStatus()
            guard accountStatus == .available else {
                return unavailableJSON(
                    status: statusName(for: accountStatus),
                    error: "iCloud account is not available."
                )
            }

            if action == "availability" {
                return successJSON(status: "available", records: [])
            }

            let database = container.privateCloudDatabase
            let zoneID = CKRecordZone.ID(
                zoneName: ticklyICloudZoneName,
                ownerName: CKCurrentUserDefaultName
            )
            try await ensureZone(database: database, zoneID: zoneID)

            var remoteRecords = try await fetchAllRecords(database: database, zoneID: zoneID)
            let localRecords = request["records"] as? [[String: Any]] ?? []
            let recordsToSave = localRecords.compactMap { localRecord -> CKRecord? in
                guard let syncID = localRecord["sync_id"] as? String else {
                    return nil
                }
                let localUpdatedAt = localRecord["updated_at"] as? String ?? ""
                if let remote = remoteRecords[syncID],
                   let remoteUpdatedAt = remote["updated_at"] as? String,
                   remoteUpdatedAt > localUpdatedAt {
                    return nil
                }

                let recordType = localRecord["record_type"] as? String ?? "TicklyV2Todo"
                let existing = remoteRecords[syncID]
                let record = existing ?? CKRecord(
                    recordType: recordType,
                    recordID: CKRecord.ID(recordName: syncID, zoneID: zoneID)
                )
                apply(localRecord: localRecord, to: record)
                return record
            }

            if !recordsToSave.isEmpty {
                let savedRecords = try await save(records: recordsToSave, database: database)
                for record in savedRecords {
                    if let syncID = record["sync_id"] as? String {
                        remoteRecords[syncID] = record
                    }
                }
            }

            let responseRecords = remoteRecords.values
                .compactMap(recordDictionary)
                .sorted { left, right in
                    (left["sync_id"] as? String ?? "") < (right["sync_id"] as? String ?? "")
                }
            return successJSON(status: "synced", records: responseRecords)
        } catch {
            return unavailableJSON(status: "error", error: error.localizedDescription)
        }
    }

    static func unavailableJSON(status: String, error: String) -> String {
        ticklyICloudUnavailableJSON(status: status, error: error)
    }

    private static func successJSON(status: String, records: [[String: Any]]) -> String {
        encode([
            "available": true,
            "status": status,
            "error": NSNull(),
            "records": records,
            "synced_at": isoNow()
        ])
    }

    private static func ensureZone(
        database: CKDatabase,
        zoneID: CKRecordZone.ID
    ) async throws {
        do {
            _ = try await fetchZone(database: database, zoneID: zoneID)
            return
        } catch {
            if let cloudKitError = error as? CKError, cloudKitError.code == .zoneNotFound {
                _ = try await save(zone: CKRecordZone(zoneID: zoneID), database: database)
                return
            }
            throw error
        }
    }

    private static func fetchZone(database: CKDatabase, zoneID: CKRecordZone.ID) async throws -> CKRecordZone {
        try await withCheckedThrowingContinuation { continuation in
            database.fetch(withRecordZoneID: zoneID) { zone, error in
                if let zone {
                    continuation.resume(returning: zone)
                    return
                }
                if let error {
                    continuation.resume(throwing: error)
                    return
                }
                continuation.resume(throwing: CKError(.zoneNotFound))
            }
        }
    }

    private static func save(zone: CKRecordZone, database: CKDatabase) async throws -> CKRecordZone {
        try await withCheckedThrowingContinuation { continuation in
            let operation = CKModifyRecordZonesOperation(recordZonesToSave: [zone], recordZoneIDsToDelete: nil)
            operation.modifyRecordZonesResultBlock = { result in
                switch result {
                case .success:
                    continuation.resume(returning: zone)
                case .failure(let error):
                    continuation.resume(throwing: error)
                }
            }
            database.add(operation)
        }
    }

    private static func fetchAllRecords(
        database: CKDatabase,
        zoneID: CKRecordZone.ID
    ) async throws -> [String: CKRecord] {
        var records: [String: CKRecord] = [:]
        for recordType in ticklyICloudRecordTypes {
            let fetched = try await fetchRecords(
                database: database,
                zoneID: zoneID,
                recordType: recordType
            )
            for record in fetched {
                if let syncID = record["sync_id"] as? String {
                    records[syncID] = record
                }
            }
        }
        return records
    }

    private static func fetchRecords(
        database: CKDatabase,
        zoneID: CKRecordZone.ID,
        recordType: String
    ) async throws -> [CKRecord] {
        let query = CKQuery(recordType: recordType, predicate: NSPredicate(value: true))
        let operation = CKQueryOperation(query: query)
        operation.zoneID = zoneID
        return try await fetchRecords(database: database, operation: operation)
    }

    private static func fetchRecords(
        database: CKDatabase,
        operation: CKQueryOperation
    ) async throws -> [CKRecord] {
        try await withCheckedThrowingContinuation { continuation in
            var records: [CKRecord] = []
            operation.recordMatchedBlock = { _, result in
                if case .success(let record) = result {
                    records.append(record)
                }
            }
            operation.queryResultBlock = { result in
                switch result {
                case .success(let cursor):
                    guard let cursor else {
                        continuation.resume(returning: records)
                        return
                    }
                    let nextOperation = CKQueryOperation(cursor: cursor)
                    Task {
                        do {
                            let nextRecords = try await fetchRecords(
                                database: database,
                                operation: nextOperation
                            )
                            continuation.resume(returning: records + nextRecords)
                        } catch {
                            continuation.resume(throwing: error)
                        }
                    }
                case .failure(let error):
                    continuation.resume(throwing: error)
                }
            }
            database.add(operation)
        }
    }

    private static func save(records: [CKRecord], database: CKDatabase) async throws -> [CKRecord] {
        try await withCheckedThrowingContinuation { continuation in
            let operation = CKModifyRecordsOperation(recordsToSave: records, recordIDsToDelete: nil)
            operation.savePolicy = .allKeys
            operation.modifyRecordsResultBlock = { result in
                switch result {
                case .success:
                    continuation.resume(returning: records)
                case .failure(let error):
                    continuation.resume(throwing: error)
                }
            }
            database.add(operation)
        }
    }

    private static func apply(localRecord: [String: Any], to record: CKRecord) {
        record["sync_id"] = localRecord["sync_id"] as? NSString
        record["entity"] = localRecord["entity"] as? NSString
        record["updated_at"] = localRecord["updated_at"] as? NSString
        if let deletedAt = localRecord["deleted_at"] as? String {
            record["deleted_at"] = deletedAt as NSString
        } else {
            record["deleted_at"] = nil
        }

        let payload = localRecord["payload"] ?? [:]
        let payloadData = (try? JSONSerialization.data(withJSONObject: payload)) ?? Data("{}".utf8)
        record["payload_json"] = String(data: payloadData, encoding: .utf8)! as NSString
    }

    private static func recordDictionary(_ record: CKRecord) -> [String: Any]? {
        guard
            let syncID = record["sync_id"] as? String,
            let entity = record["entity"] as? String,
            let updatedAt = record["updated_at"] as? String
        else {
            return nil
        }

        let payloadJSON = record["payload_json"] as? String ?? "{}"
        let payload = payloadJSON.data(using: .utf8)
            .flatMap { try? JSONSerialization.jsonObject(with: $0) }
            ?? [:]
        return [
            "record_type": record.recordType,
            "entity": entity,
            "sync_id": syncID,
            "updated_at": updatedAt,
            "deleted_at": record["deleted_at"] as Any? ?? NSNull(),
            "payload": payload
        ]
    }

    private static func statusName(for status: CKAccountStatus) -> String {
        switch status {
        case .available:
            return "available"
        case .couldNotDetermine:
            return "could_not_determine"
        case .noAccount:
            return "no_account"
        case .restricted:
            return "restricted"
        case .temporarilyUnavailable:
            return "temporarily_unavailable"
        @unknown default:
            return "unknown"
        }
    }

    private static func isoNow() -> String {
        let formatter = ISO8601DateFormatter()
        formatter.formatOptions = [.withInternetDateTime]
        return formatter.string(from: Date())
    }

    private static func encode(_ value: [String: Any]) -> String {
        let data = (try? JSONSerialization.data(withJSONObject: value)) ?? Data("{}".utf8)
        return String(data: data, encoding: .utf8) ?? "{}"
    }
}

private func ticklyEncodeJSONObject(_ value: [String: Any]) -> String {
    let data = (try? JSONSerialization.data(withJSONObject: value)) ?? Data("{}".utf8)
    return String(data: data, encoding: .utf8) ?? "{}"
}
