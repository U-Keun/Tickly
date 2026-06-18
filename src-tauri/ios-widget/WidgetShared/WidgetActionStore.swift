import Foundation

private struct WidgetQueuedAction: Codable {
    let itemId: Int64
    let queuedAt: String
}

enum WidgetActionStore {
    static func toggleItemAndQueue(itemId: Int64, categoryId: Int64?) {
        queueToggleAction(itemId: itemId)
        applyToggleToSnapshot(itemId: itemId, categoryId: categoryId)
    }

    private static func applyToggleToSnapshot(itemId: Int64, categoryId: Int64?) {
        guard
            let cacheURL = WidgetSnapshotLoader.cacheURL(),
            let data = try? Data(contentsOf: cacheURL),
            var snapshot = try? JSONDecoder().decode(WidgetSnapshot.self, from: data)
        else {
            return
        }

        guard applyToggle(itemId: itemId, categoryId: categoryId, snapshot: &snapshot) else {
            return
        }

        let encoder = JSONEncoder()
        if let updatedData = try? encoder.encode(snapshot) {
            try? updatedData.write(to: cacheURL, options: .atomic)
        }
    }

    private static func applyToggle(
        itemId: Int64,
        categoryId: Int64?,
        snapshot: inout WidgetSnapshot
    ) -> Bool {
        guard let categoryIndex = categoryIndex(
            for: itemId,
            categoryId: categoryId,
            categories: snapshot.categories
        ) else {
            return false
        }

        var category = snapshot.categories[categoryIndex]
        var changed = false
        let hadPendingItem = category.pendingItemIds.contains(itemId)

        if hadPendingItem {
            category.pendingItemIds.removeAll { $0 == itemId }
            category.pendingItems.removeAll { $0.id == itemId }
            category.pendingCount = max(0, category.pendingCount - 1)
            snapshot.pendingCount = max(0, snapshot.pendingCount - 1)
            changed = true
        }

        if let itemIndex = snapshot.items.firstIndex(where: { $0.id == itemId }) {
            if snapshot.items[itemIndex].done == false {
                snapshot.items[itemIndex].done = true
                changed = true
                if !hadPendingItem {
                    category.pendingCount = max(0, category.pendingCount - 1)
                    snapshot.pendingCount = max(0, snapshot.pendingCount - 1)
                }
            }
        }

        if changed {
            category.firstPendingItemId = category.pendingItems.first?.id ?? category.pendingItemIds.first
            snapshot.categories[categoryIndex] = category
        }

        return changed
    }

    private static func categoryIndex(
        for itemId: Int64,
        categoryId: Int64?,
        categories: [WidgetCategorySummary]
    ) -> Int? {
        if let categoryId {
            if let index = categories.firstIndex(where: { $0.categoryId == categoryId }) {
                return index
            }
        } else if let index = categories.firstIndex(where: { $0.categoryId == nil }) {
            return index
        }

        return categories.firstIndex(where: { $0.firstPendingItemId == itemId })
    }

    private static func queueToggleAction(itemId: Int64) {
        guard let queueURL = WidgetSnapshotLoader.actionQueueURL() else {
            return
        }

        let queuedAt = ISO8601DateFormatter().string(from: Date())
        let action = WidgetQueuedAction(itemId: itemId, queuedAt: queuedAt)
        var queue = readQueue(from: queueURL)
        queue.append(action)

        let encoder = JSONEncoder()
        if let data = try? encoder.encode(queue) {
            try? data.write(to: queueURL, options: .atomic)
        }
    }

    private static func readQueue(from queueURL: URL) -> [WidgetQueuedAction] {
        guard
            let data = try? Data(contentsOf: queueURL),
            let queue = try? JSONDecoder().decode([WidgetQueuedAction].self, from: data)
        else {
            return []
        }

        return queue
    }
}
