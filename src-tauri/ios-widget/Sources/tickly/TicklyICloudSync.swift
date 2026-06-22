import CloudKit
import Foundation
import WebKit

private let ticklyICloudContainerIdentifier = "iCloud.com.u-keunsong.tickly"
private let ticklyICloudZoneName = "TicklyChecklist"

private struct TicklyICloudSyncRequest: Decodable {
    let token: String
    let records: [TicklyICloudSyncRecord]
}

private struct TicklyICloudSyncRecord: Codable {
    let entityType: String
    let syncId: String
    let updatedAt: String
    let deletedAt: String?
    let payload: TicklyJSONValue
}

private struct TicklyICloudSyncResult: Encodable {
    let token: String
    let status: String
    let message: String?
    let remoteRecords: [TicklyICloudSyncRecord]
    let syncedSyncIds: [String]
}

private enum TicklyJSONValue: Codable {
    case string(String)
    case number(Double)
    case bool(Bool)
    case object([String: TicklyJSONValue])
    case array([TicklyJSONValue])
    case null

    init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        if container.decodeNil() {
            self = .null
        } else if let value = try? container.decode(Bool.self) {
            self = .bool(value)
        } else if let value = try? container.decode(Double.self) {
            self = .number(value)
        } else if let value = try? container.decode(String.self) {
            self = .string(value)
        } else if let value = try? container.decode([String: TicklyJSONValue].self) {
            self = .object(value)
        } else if let value = try? container.decode([TicklyJSONValue].self) {
            self = .array(value)
        } else {
            self = .null
        }
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        switch self {
        case .string(let value):
            try container.encode(value)
        case .number(let value):
            try container.encode(value)
        case .bool(let value):
            try container.encode(value)
        case .object(let value):
            try container.encode(value)
        case .array(let value):
            try container.encode(value)
        case .null:
            try container.encodeNil()
        }
    }
}

@_cdecl("tickly_exchange_icloud_sync")
public func ticklyExchangeICloudSync(
    webViewRaw: UnsafeMutableRawPointer?,
    viewControllerRaw: UnsafeMutableRawPointer?,
    requestCString: UnsafePointer<CChar>?
) -> Bool {
    guard
        let webViewRaw,
        let requestCString
    else {
        return false
    }

    let webView = Unmanaged<WKWebView>.fromOpaque(webViewRaw).takeUnretainedValue()
    let requestString = String(cString: requestCString)
    guard
        let requestData = requestString.data(using: .utf8),
        let request = try? JSONDecoder().decode(TicklyICloudSyncRequest.self, from: requestData)
    else {
        return false
    }

    if #available(iOS 17.0, *) {
        TicklyICloudSyncBridge.shared.exchange(request: request, webView: webView)
        return true
    }

    TicklyICloudSyncBridge.emit(
        TicklyICloudSyncResult(
            token: request.token,
            status: "unavailable",
            message: "iCloud sync requires iOS 17 or later.",
            remoteRecords: [],
            syncedSyncIds: []
        ),
        webView: webView
    )
    _ = viewControllerRaw
    return true
}

private final class TicklyICloudSyncBridge {
    static let shared = TicklyICloudSyncBridge()

    private let container = CKContainer(identifier: ticklyICloudContainerIdentifier)
    private lazy var database = container.privateCloudDatabase
    private let zoneID = CKRecordZone.ID(
        zoneName: ticklyICloudZoneName,
        ownerName: CKCurrentUserDefaultName
    )

    private init() {}

    func exchange(request: TicklyICloudSyncRequest, webView: WKWebView) {
        container.accountStatus { [weak self, weak webView] accountStatus, error in
            guard let self, let webView else { return }

            if let error {
                Self.emit(
                    TicklyICloudSyncResult(
                        token: request.token,
                        status: "error",
                        message: error.localizedDescription,
                        remoteRecords: [],
                        syncedSyncIds: []
                    ),
                    webView: webView
                )
                return
            }

            guard accountStatus == .available else {
                Self.emit(
                    TicklyICloudSyncResult(
                        token: request.token,
                        status: "accountUnavailable",
                        message: "No available iCloud account.",
                        remoteRecords: [],
                        syncedSyncIds: []
                    ),
                    webView: webView
                )
                return
            }

            self.ensureZone { zoneResult in
                switch zoneResult {
                case .failure(let error):
                    Self.emit(
                        TicklyICloudSyncResult(
                            token: request.token,
                            status: "error",
                            message: error.localizedDescription,
                            remoteRecords: [],
                            syncedSyncIds: []
                        ),
                        webView: webView
                    )
                case .success:
                    self.fetchRemoteRecords { fetchResult in
                        switch fetchResult {
                        case .failure(let error):
                            Self.emit(
                                TicklyICloudSyncResult(
                                    token: request.token,
                                    status: "error",
                                    message: error.localizedDescription,
                                    remoteRecords: [],
                                    syncedSyncIds: []
                                ),
                                webView: webView
                            )
                        case .success(let remote):
                            self.merge(request: request, remote: remote, webView: webView)
                        }
                    }
                }
            }
        }
    }

    private func merge(
        request: TicklyICloudSyncRequest,
        remote: [String: (record: TicklyICloudSyncRecord, cloudRecord: CKRecord)],
        webView: WKWebView
    ) {
        var remoteRecordsToApply: [TicklyICloudSyncRecord] = []
        var cloudRecordsToSave: [CKRecord] = []
        var syncedSyncIds: [String] = []
        let localById = Dictionary(uniqueKeysWithValues: request.records.map { ($0.syncId, $0) })

        for localRecord in request.records {
            if let remoteRecord = remote[localRecord.syncId]?.record {
                if remoteRecord.updatedAt > localRecord.updatedAt {
                    remoteRecordsToApply.append(remoteRecord)
                } else {
                    let cloudRecord = remote[localRecord.syncId]?.cloudRecord
                        ?? makeCloudRecord(for: localRecord)
                    apply(localRecord, to: cloudRecord)
                    cloudRecordsToSave.append(cloudRecord)
                    syncedSyncIds.append(localRecord.syncId)
                }
            } else {
                let cloudRecord = makeCloudRecord(for: localRecord)
                apply(localRecord, to: cloudRecord)
                cloudRecordsToSave.append(cloudRecord)
                syncedSyncIds.append(localRecord.syncId)
            }
        }

        for (syncId, remoteRecord) in remote {
            if localById[syncId] == nil {
                remoteRecordsToApply.append(remoteRecord.record)
            }
        }

        save(cloudRecordsToSave) { result in
            switch result {
            case .failure(let error):
                Self.emit(
                    TicklyICloudSyncResult(
                        token: request.token,
                        status: "error",
                        message: error.localizedDescription,
                        remoteRecords: [],
                        syncedSyncIds: []
                    ),
                    webView: webView
                )
            case .success:
                Self.emit(
                    TicklyICloudSyncResult(
                        token: request.token,
                        status: "success",
                        message: nil,
                        remoteRecords: remoteRecordsToApply,
                        syncedSyncIds: syncedSyncIds
                    ),
                    webView: webView
                )
            }
        }
    }

    private func ensureZone(completion: @escaping (Result<Void, Error>) -> Void) {
        database.fetch(withRecordZoneID: zoneID) { [weak self] _, error in
            guard let self else { return }
            if error == nil {
                completion(.success(()))
                return
            }

            let zone = CKRecordZone(zoneID: self.zoneID)
            let operation = CKModifyRecordZonesOperation(
                recordZonesToSave: [zone],
                recordZoneIDsToDelete: nil
            )
            operation.modifyRecordZonesResultBlock = { result in
                switch result {
                case .failure(let error):
                    completion(.failure(error))
                case .success:
                    completion(.success(()))
                }
            }
            self.database.add(operation)
        }
    }

    private func fetchRemoteRecords(
        completion: @escaping (Result<[String: (record: TicklyICloudSyncRecord, cloudRecord: CKRecord)], Error>) -> Void
    ) {
        fetchRemoteRecords(
            changeToken: nil,
            merged: [:],
            didRetryExpiredToken: false,
            completion: completion
        )
    }

    private func fetchRemoteRecords(
        changeToken: CKServerChangeToken?,
        merged: [String: (record: TicklyICloudSyncRecord, cloudRecord: CKRecord)],
        didRetryExpiredToken: Bool,
        completion: @escaping (Result<[String: (record: TicklyICloudSyncRecord, cloudRecord: CKRecord)], Error>) -> Void
    ) {
        let configuration = CKFetchRecordZoneChangesOperation.ZoneConfiguration(
            previousServerChangeToken: changeToken,
            resultsLimit: nil,
            desiredKeys: nil
        )
        let operation = CKFetchRecordZoneChangesOperation(
            recordZoneIDs: [zoneID],
            configurationsByRecordZoneID: [zoneID: configuration]
        )
        let lock = NSLock()
        var nextMerged = merged
        var nextChangeToken: CKServerChangeToken?
        var moreComing = false
        var fetchError: Error?

        operation.recordWasChangedBlock = { [weak self] _, result in
            guard let self else { return }
            switch result {
            case .failure(let error):
                lock.lock()
                fetchError = fetchError ?? error
                lock.unlock()
            case .success(let record):
                guard let syncRecord = self.syncRecord(from: record) else {
                    return
                }
                lock.lock()
                nextMerged[syncRecord.syncId] = (syncRecord, record)
                lock.unlock()
            }
        }

        operation.recordZoneFetchResultBlock = { _, result in
            lock.lock()
            defer { lock.unlock() }

            switch result {
            case .failure(let error):
                fetchError = fetchError ?? error
            case .success(let result):
                nextChangeToken = result.serverChangeToken
                moreComing = result.moreComing
            }
        }

        operation.fetchRecordZoneChangesResultBlock = { [weak self] result in
            guard let self else { return }

            lock.lock()
            let completedMerged = nextMerged
            let completedToken = nextChangeToken
            let shouldContinue = moreComing
            let completedError = fetchError
            lock.unlock()

            if let completedError {
                if self.isChangeTokenExpired(completedError), !didRetryExpiredToken {
                    self.clearStoredZoneChangeToken()
                    self.fetchRemoteRecords(
                        changeToken: nil,
                        merged: [:],
                        didRetryExpiredToken: true,
                        completion: completion
                    )
                    return
                }
                completion(.failure(completedError))
                return
            }

            switch result {
            case .failure(let error):
                if self.isChangeTokenExpired(error), !didRetryExpiredToken {
                    self.clearStoredZoneChangeToken()
                    self.fetchRemoteRecords(
                        changeToken: nil,
                        merged: [:],
                        didRetryExpiredToken: true,
                        completion: completion
                    )
                    return
                }
                completion(.failure(error))
            case .success:
                if shouldContinue, let completedToken {
                    self.fetchRemoteRecords(
                        changeToken: completedToken,
                        merged: completedMerged,
                        didRetryExpiredToken: didRetryExpiredToken,
                        completion: completion
                    )
                } else {
                    _ = completedToken
                    completion(.success(completedMerged))
                }
            }
        }

        database.add(operation)
    }

    private func clearStoredZoneChangeToken() {
        // The foreground pilot intentionally performs full-zone fetches, so there is no
        // persisted change token to clear. The method stays as a no-op for retry flow symmetry.
    }

    private func isChangeTokenExpired(_ error: Error) -> Bool {
        guard let ckError = error as? CKError else {
            return false
        }
        return ckError.code == .changeTokenExpired
    }

    private func save(_ records: [CKRecord], completion: @escaping (Result<Void, Error>) -> Void) {
        guard !records.isEmpty else {
            completion(.success(()))
            return
        }

        let operation = CKModifyRecordsOperation(recordsToSave: records, recordIDsToDelete: nil)
        operation.savePolicy = .allKeys
        operation.modifyRecordsResultBlock = { result in
            switch result {
            case .failure(let error):
                completion(.failure(error))
            case .success:
                completion(.success(()))
            }
        }
        database.add(operation)
    }

    private func makeCloudRecord(for record: TicklyICloudSyncRecord) -> CKRecord {
        CKRecord(
            recordType: recordType(for: record.entityType),
            recordID: CKRecord.ID(recordName: record.syncId, zoneID: zoneID)
        )
    }

    private func apply(_ syncRecord: TicklyICloudSyncRecord, to cloudRecord: CKRecord) {
        cloudRecord["entityType"] = syncRecord.entityType as NSString
        cloudRecord["updatedAt"] = syncRecord.updatedAt as NSString
        if let deletedAt = syncRecord.deletedAt {
            cloudRecord["deletedAt"] = deletedAt as NSString
        } else {
            cloudRecord["deletedAt"] = nil
        }
        let payloadData = (try? JSONEncoder().encode(syncRecord.payload)) ?? Data("{}".utf8)
        let payload = String(data: payloadData, encoding: .utf8) ?? "{}"
        cloudRecord["payload"] = payload as NSString
    }

    private func syncRecord(from record: CKRecord) -> TicklyICloudSyncRecord? {
        guard
            let entityType = record["entityType"] as? String,
            let updatedAt = record["updatedAt"] as? String
        else {
            return nil
        }

        let payloadString = (record["payload"] as? String) ?? "{}"
        let payloadData = payloadString.data(using: .utf8) ?? Data("{}".utf8)
        let payload = (try? JSONDecoder().decode(TicklyJSONValue.self, from: payloadData)) ?? .object([:])

        return TicklyICloudSyncRecord(
            entityType: entityType,
            syncId: record.recordID.recordName,
            updatedAt: updatedAt,
            deletedAt: record["deletedAt"] as? String,
            payload: payload
        )
    }

    private func recordType(for entityType: String) -> String {
        switch entityType {
        case "category":
            return "TicklyCategory"
        case "todo":
            return "TicklyTodo"
        case "tag":
            return "TicklyTag"
        case "todo_tag":
            return "TicklyTodoTag"
        case "completion_log":
            return "TicklyCompletionLog"
        case "setting":
            return "TicklySetting"
        default:
            return "TicklyTodo"
        }
    }

    static func emit(_ result: TicklyICloudSyncResult, webView: WKWebView) {
        let data = (try? JSONEncoder().encode(result)) ?? Data()
        let json = String(data: data, encoding: .utf8) ?? "{}"
        DispatchQueue.main.async {
            webView.evaluateJavaScript(
                "window.dispatchEvent(new CustomEvent(\"tickly:iCloudSyncResult\", { detail: \(json) }));",
                completionHandler: nil
            )
        }
    }
}
