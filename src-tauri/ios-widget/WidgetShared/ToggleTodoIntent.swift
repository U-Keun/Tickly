import AppIntents
import WidgetKit

@available(iOS 17.0, iOSApplicationExtension 17.0, *)
struct ToggleTodoIntent: AppIntent {
    static var title: LocalizedStringResource = "Toggle Todo"
    static var description = IntentDescription("Marks the next pending task in a category as done.")
    static var openAppWhenRun = false

    @Parameter(title: "Item ID")
    var itemId: Int

    @Parameter(title: "Category ID")
    var categoryId: Int?

    init() {}

    init(itemId: Int, categoryId: Int?) {
        self.itemId = itemId
        self.categoryId = categoryId
    }

    func perform() async throws -> some IntentResult {
        let normalizedCategoryId = categoryId.map { Int64($0) }
        WidgetActionStore.toggleItemAndQueue(itemId: Int64(itemId), categoryId: normalizedCategoryId)
        WidgetCenter.shared.reloadAllTimelines()
        return .result()
    }
}
