import AppIntents

@available(iOS 17.0, iOSApplicationExtension 17.0, *)
enum WidgetCategoryIdentifier {
    static let uncategorized = "uncategorized"

    static func make(from categoryId: Int64?) -> String {
        if let categoryId {
            return String(categoryId)
        }
        return uncategorized
    }
}

@available(iOS 17.0, iOSApplicationExtension 17.0, *)
struct WidgetCategoryEntity: AppEntity {
    static var typeDisplayRepresentation: TypeDisplayRepresentation = TypeDisplayRepresentation(
        name: "Category"
    )
    static var defaultQuery = WidgetCategoryQuery()

    let id: String
    let categoryId: Int64?
    let categoryName: String

    var displayRepresentation: DisplayRepresentation {
        DisplayRepresentation(title: LocalizedStringResource(stringLiteral: categoryName))
    }

    init(categoryId: Int64?, categoryName: String) {
        self.id = WidgetCategoryIdentifier.make(from: categoryId)
        self.categoryId = categoryId
        self.categoryName = categoryName
    }
}

@available(iOS 17.0, iOSApplicationExtension 17.0, *)
struct WidgetCategoryQuery: EntityQuery {
    func entities(for identifiers: [WidgetCategoryEntity.ID]) async throws -> [WidgetCategoryEntity] {
        let all = categoryEntities()
        let byId = Dictionary(uniqueKeysWithValues: all.map { ($0.id, $0) })
        return identifiers.compactMap { byId[$0] }
    }

    func suggestedEntities() async throws -> [WidgetCategoryEntity] {
        categoryEntities()
    }

    private func categoryEntities() -> [WidgetCategoryEntity] {
        let snapshot = WidgetSnapshotLoader.load()
        return snapshot.categories.map { category in
            WidgetCategoryEntity(categoryId: category.categoryId, categoryName: category.categoryName)
        }
    }
}

@available(iOS 17.0, iOSApplicationExtension 17.0, *)
struct CategoryWidgetConfigurationIntent: WidgetConfigurationIntent {
    static var title: LocalizedStringResource = "Tickly Category"
    static var description = IntentDescription("Show and check tasks for one category.")

    @Parameter(title: "Category")
    var category: WidgetCategoryEntity?

    static var parameterSummary: some ParameterSummary {
        Summary("Category: \(\.$category)")
    }
}
