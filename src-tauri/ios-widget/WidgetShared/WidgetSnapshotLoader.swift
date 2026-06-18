import Foundation

enum WidgetSnapshotLoader {
    static let cacheFileName = "widget-cache.json"
    static let actionFileName = "widget-actions.json"
    static let defaultAppGroupId = "group.com.u-keunsong.tickly"

    static func load() -> WidgetSnapshot {
        guard
            let cacheURL = cacheURL(),
            let data = try? Data(contentsOf: cacheURL),
            let snapshot = try? JSONDecoder().decode(WidgetSnapshot.self, from: data)
        else {
            return .empty
        }

        return snapshot
    }

    static func cacheURL() -> URL? {
        let appGroupId = configuredAppGroupId()
        guard !appGroupId.isEmpty else { return nil }

        return FileManager.default
            .containerURL(forSecurityApplicationGroupIdentifier: appGroupId)?
            .appendingPathComponent(cacheFileName, isDirectory: false)
    }

    static func actionQueueURL() -> URL? {
        let appGroupId = configuredAppGroupId()
        guard !appGroupId.isEmpty else { return nil }

        return FileManager.default
            .containerURL(forSecurityApplicationGroupIdentifier: appGroupId)?
            .appendingPathComponent(actionFileName, isDirectory: false)
    }

    private static func configuredAppGroupId() -> String {
        let configured = (Bundle.main.object(forInfoDictionaryKey: "TICKLY_APP_GROUP_ID") as? String)?
            .trimmingCharacters(in: .whitespacesAndNewlines)

        if let configured, !configured.isEmpty {
            return configured
        }

        return defaultAppGroupId
    }
}
