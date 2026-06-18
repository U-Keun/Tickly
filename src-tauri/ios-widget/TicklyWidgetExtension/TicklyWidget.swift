import SwiftUI
import WidgetKit

@available(iOSApplicationExtension 17.0, *)
struct TicklyTimelineEntry: TimelineEntry {
    let date: Date
    let snapshot: WidgetSnapshot
    let selectedCategoryIdentifier: String?
}

@available(iOSApplicationExtension 17.0, *)
struct TicklyTimelineProvider: AppIntentTimelineProvider {
    typealias Intent = CategoryWidgetConfigurationIntent

    func placeholder(in context: Context) -> TicklyTimelineEntry {
        TicklyTimelineEntry(
            date: Date(),
            snapshot: WidgetSnapshot(
                generatedAt: "",
                totalCount: 7,
                pendingCount: 4,
                items: [],
                categories: [
                    WidgetCategorySummary(
                        categoryId: 1,
                        categoryName: "Essentials",
                        totalCount: 4,
                        pendingCount: 3,
                        firstPendingItemId: 101,
                        pendingItemIds: [101, 102, 103],
                        pendingItems: [
                            WidgetCategoryPendingItem(id: 101, text: "Wallet", displayOrder: 1000, tags: ["Daily"]),
                            WidgetCategoryPendingItem(id: 102, text: "Keys", displayOrder: 2000, tags: ["Home"]),
                            WidgetCategoryPendingItem(id: 103, text: "Phone charger", displayOrder: 3000, tags: ["Electronics"]),
                        ]
                    ),
                    WidgetCategorySummary(
                        categoryId: 2,
                        categoryName: "Work",
                        totalCount: 3,
                        pendingCount: 1,
                        firstPendingItemId: 201,
                        pendingItemIds: [201],
                        pendingItems: [
                            WidgetCategoryPendingItem(id: 201, text: "Prepare report", displayOrder: 1000, tags: ["Work"]),
                        ]
                    ),
                ]
            ),
            selectedCategoryIdentifier: WidgetCategoryIdentifier.make(from: 1)
        )
    }

    func snapshot(
        for configuration: CategoryWidgetConfigurationIntent,
        in context: Context
    ) async -> TicklyTimelineEntry {
        makeEntry(configuration: configuration)
    }

    func timeline(
        for configuration: CategoryWidgetConfigurationIntent,
        in context: Context
    ) async -> Timeline<TicklyTimelineEntry> {
        let entry = makeEntry(configuration: configuration)
        let refreshDate = Calendar.current.date(byAdding: .minute, value: 15, to: Date())
            ?? Date().addingTimeInterval(900)
        return Timeline(entries: [entry], policy: .after(refreshDate))
    }

    private func makeEntry(configuration: CategoryWidgetConfigurationIntent) -> TicklyTimelineEntry {
        TicklyTimelineEntry(
            date: Date(),
            snapshot: WidgetSnapshotLoader.load(),
            selectedCategoryIdentifier: configuration.category?.id
        )
    }
}

@available(iOSApplicationExtension 17.0, *)
struct TicklyWidgetEntryView: View {
    @Environment(\.widgetFamily) private var family
    var entry: TicklyTimelineProvider.Entry

    private var selectedCategory: WidgetCategorySummary? {
        if let selectedCategoryIdentifier = entry.selectedCategoryIdentifier {
            if let category = entry.snapshot.categories.first(
                where: { WidgetCategoryIdentifier.make(from: $0.categoryId) == selectedCategoryIdentifier }
            ) {
                return category
            }
        }

        return entry.snapshot.categories.first
    }

    private var maxVisibleItems: Int {
        if family == .systemLarge {
            return 7
        }
        if family == .systemSmall {
            return 3
        }
        return 3
    }

    private var horizontalInset: CGFloat {
        if family == .systemLarge {
            return 18
        }
        if family == .systemSmall {
            return 8
        }
        return 14
    }

    private var topInset: CGFloat {
        if family == .systemLarge {
            return 16
        }
        if family == .systemSmall {
            return 8
        }
        return 18
    }

    private var bottomInset: CGFloat {
        if family == .systemLarge {
            return 16
        }
        if family == .systemSmall {
            return 4
        }
        return 12
    }

    private var headerHeight: CGFloat {
        if family == .systemLarge {
            return 44
        }
        if family == .systemSmall {
            return 26
        }
        return 40
    }

    private var headerFrameAlignment: Alignment {
        if family == .systemLarge {
            return .topLeading
        }
        return .leading
    }

    private var completionRate: Int {
        guard let selectedCategory, selectedCategory.totalCount > 0 else {
            return 0
        }

        let doneCount = selectedCategory.totalCount - selectedCategory.pendingCount
        let completion = Double(doneCount) / Double(selectedCategory.totalCount) * 100
        return Int(completion.rounded())
    }

    private func completedCount(for category: WidgetCategorySummary) -> Int {
        max(0, category.totalCount - category.pendingCount)
    }

    private var widgetTheme: WidgetTheme {
        entry.snapshot.theme
    }

    private var backgroundTopColor: Color {
        color(from: widgetTheme.paper, fallback: .white)
    }

    private var backgroundBottomColor: Color {
        color(from: widgetTheme.canvas, fallback: Color(white: 0.94))
    }

    private var primaryTextColor: Color {
        color(from: widgetTheme.ink, fallback: .primary)
    }

    private var secondaryTextColor: Color {
        color(from: widgetTheme.inkMuted, fallback: .secondary)
    }

    private var accentColor: Color {
        color(from: widgetTheme.accentSkyStrong, fallback: .blue)
    }

    private var tagTextColor: Color {
        color(from: widgetTheme.accentSkyStrong, fallback: secondaryTextColor)
    }

    private var tagBackgroundColor: Color {
        color(from: widgetTheme.accentSky, fallback: secondaryTextColor).opacity(0.24)
    }

    private var checkboxBorderColor: Color {
        color(from: widgetTheme.stroke, fallback: secondaryTextColor)
    }

    private var checkboxFillColor: Color {
        color(from: widgetTheme.paper, fallback: .clear)
    }

    private var widgetBackground: LinearGradient {
        LinearGradient(
            colors: [backgroundTopColor, backgroundBottomColor],
            startPoint: .topLeading,
            endPoint: .bottomTrailing
        )
    }

    private func color(from hex: String, fallback: Color) -> Color {
        Self.colorFromHex(hex) ?? fallback
    }

    private static func colorFromHex(_ hex: String) -> Color? {
        let trimmed = hex.trimmingCharacters(in: .whitespacesAndNewlines)
        guard trimmed.hasPrefix("#") else {
            return nil
        }

        let rawHex = String(trimmed.dropFirst())
        guard rawHex.count == 6 || rawHex.count == 8 else {
            return nil
        }

        var value: UInt64 = 0
        guard Scanner(string: rawHex).scanHexInt64(&value) else {
            return nil
        }

        let red: Double
        let green: Double
        let blue: Double
        let alpha: Double

        if rawHex.count == 8 {
            red = Double((value & 0xFF00_0000) >> 24) / 255
            green = Double((value & 0x00FF_0000) >> 16) / 255
            blue = Double((value & 0x0000_FF00) >> 8) / 255
            alpha = Double(value & 0x0000_00FF) / 255
        } else {
            red = Double((value & 0xFF00_00) >> 16) / 255
            green = Double((value & 0x00FF_00) >> 8) / 255
            blue = Double(value & 0x0000_FF) / 255
            alpha = 1
        }

        return Color(.sRGB, red: red, green: green, blue: blue, opacity: alpha)
    }

    @ViewBuilder
    private func pendingCheckbox(opacity: Double = 1) -> some View {
        RoundedRectangle(cornerRadius: 4, style: .continuous)
            .fill(checkboxFillColor.opacity(0.95))
            .overlay(
                RoundedRectangle(cornerRadius: 4, style: .continuous)
                    .stroke(checkboxBorderColor, lineWidth: 1.8)
            )
            .frame(width: 16, height: 16)
            .opacity(opacity)
    }

    var body: some View {
        VStack(alignment: .leading, spacing: family == .systemSmall ? 6 : 8) {
            if let selectedCategory {
                if family == .systemSmall {
                    smallCategoryContent(category: selectedCategory)
                        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
                } else {
                    header(category: selectedCategory)
                        .frame(height: headerHeight, alignment: headerFrameAlignment)
                        .frame(maxWidth: .infinity, alignment: .topLeading)

                    categoryContent(category: selectedCategory)
                        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
                }
            } else {
                Text(WidgetCopy.noCategories)
                    .font(.subheadline)
                    .foregroundColor(secondaryTextColor)
                Spacer(minLength: 0)
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
        .padding(.horizontal, horizontalInset)
        .padding(.top, topInset)
        .padding(.bottom, bottomInset)
        .containerBackground(for: .widget) {
            widgetBackground
        }
    }

    @ViewBuilder
    private func header(
        category: WidgetCategorySummary,
        showSummary: Bool = true,
        showCompletionRate: Bool = true
    ) -> some View {
        HStack(alignment: .center) {
            HStack(spacing: 6) {
                Text(category.categoryName)
                    .font(.headline)
                    .lineLimit(1)
                    .padding(.leading, -1)
                    .foregroundColor(primaryTextColor)

                Button(intent: RefreshWidgetIntent()) {
                    Image(systemName: "arrow.clockwise")
                        .font(.caption2)
                        .foregroundColor(secondaryTextColor)
                }
                .buttonStyle(.plain)
            }
            Spacer()
            if showSummary {
                HStack(alignment: .firstTextBaseline, spacing: 6) {
                    if showCompletionRate {
                        Text(WidgetCopy.donePercent(completionRate))
                            .font(.caption)
                            .foregroundColor(secondaryTextColor)
                    }
                    Text("\(completedCount(for: category))/\(category.totalCount)")
                        .font(.headline)
                        .foregroundColor(primaryTextColor)
                }
            }
        }
    }

    @ViewBuilder
    private func smallCategoryContent(category: WidgetCategorySummary) -> some View {
        let visibleItems = Array(category.pendingItems.prefix(maxVisibleItems))
        let overflowCount = max(0, category.pendingItems.count - maxVisibleItems)

        VStack(alignment: .leading, spacing: 4) {
            header(category: category, showSummary: true, showCompletionRate: false)
                .frame(height: headerHeight, alignment: .leading)
                .frame(maxWidth: .infinity, alignment: .topLeading)

            if category.pendingItems.isEmpty {
                statusRow(text: WidgetCopy.allDone)

                if maxVisibleItems > 1 {
                    ForEach(0..<(maxVisibleItems - 1), id: \.self) { _ in
                        emptyRow()
                    }
                }
            } else {
                ForEach(0..<maxVisibleItems, id: \.self) { index in
                    if index < visibleItems.count {
                        pendingItemRow(
                            item: visibleItems[index],
                            categoryId: category.categoryId,
                            showTag: false
                        )
                    } else {
                        emptyRow()
                    }
                }
            }

            Spacer(minLength: 0)

            if overflowCount > 0 {
                overflowRow(overflowCount: overflowCount)
            }
        }
    }

    @ViewBuilder
    private func categoryContent(category: WidgetCategorySummary) -> some View {
        let visibleItems = Array(category.pendingItems.prefix(maxVisibleItems))
        let overflowCount = max(0, category.pendingItems.count - maxVisibleItems)

        VStack(alignment: .leading, spacing: 6) {
            if category.pendingItems.isEmpty {
                statusRow(text: WidgetCopy.allDone)

                if maxVisibleItems > 1 {
                    ForEach(0..<(maxVisibleItems - 1), id: \.self) { _ in
                        emptyRow()
                    }
                }
            } else {
                ForEach(0..<maxVisibleItems, id: \.self) { index in
                    if index < visibleItems.count {
                        pendingItemRow(item: visibleItems[index], categoryId: category.categoryId)
                    } else {
                        emptyRow()
                    }
                }
            }

            Spacer(minLength: 0)
            overflowRow(overflowCount: overflowCount)
        }
    }

    @ViewBuilder
    private func statusRow(text: String) -> some View {
        HStack(spacing: 8) {
            Image(systemName: "checkmark.circle.fill")
                .font(.subheadline)
                .foregroundColor(accentColor)

            Text(text)
                .font(.caption)
                .lineLimit(1)
                .foregroundColor(primaryTextColor)

            Spacer(minLength: 0)
        }
    }

    @ViewBuilder
    private func overflowRow(overflowCount: Int) -> some View {
        HStack {
            Spacer(minLength: 0)
            Text(overflowCount > 0 ? WidgetCopy.more(overflowCount) : " ")
                .font(.caption2)
                .foregroundColor(secondaryTextColor)
                .opacity(overflowCount > 0 ? 1 : 0)
        }
        .frame(maxWidth: .infinity)
    }

    @ViewBuilder
    private func emptyRow() -> some View {
        HStack(spacing: 8) {
            pendingCheckbox(opacity: 0)

            Text(" ")
                .font(.caption)
                .lineLimit(1)
                .opacity(0)

            Spacer(minLength: 0)
        }
    }

    private func compactTagLabel(for item: WidgetCategoryPendingItem) -> String {
        let normalizedTags = item.tags
            .map { $0.trimmingCharacters(in: .whitespacesAndNewlines) }
            .filter { !$0.isEmpty }

        guard let firstTag = normalizedTags.first else {
            return ""
        }

        let overflowCount = normalizedTags.count - 1
        if overflowCount > 0 {
            return "#\(firstTag) +\(overflowCount)"
        }

        return "#\(firstTag)"
    }

    @ViewBuilder
    private func pendingItemRow(
        item: WidgetCategoryPendingItem,
        categoryId: Int64?,
        showTag: Bool = true
    ) -> some View {
        let itemTagLabel = showTag ? compactTagLabel(for: item) : ""

        HStack(spacing: 8) {
            Button(
                intent: ToggleTodoIntent(
                    itemId: Int(item.id),
                    categoryId: categoryId.map { Int($0) }
                )
            ) {
                pendingCheckbox()
            }
            .buttonStyle(.plain)

            Text(item.text)
                .font(.caption)
                .lineLimit(1)
                .minimumScaleFactor(0.75)
                .truncationMode(.tail)
                .frame(maxWidth: .infinity, alignment: .leading)
                .foregroundColor(primaryTextColor)

            if showTag && !itemTagLabel.isEmpty {
                Text(itemTagLabel)
                    .font(.caption2)
                    .foregroundColor(tagTextColor)
                    .lineLimit(1)
                    .fixedSize(horizontal: true, vertical: false)
                    .padding(.horizontal, 6)
                    .padding(.vertical, 2)
                    .background(tagBackgroundColor, in: Capsule())
            }
        }
        .padding(.horizontal, family == .systemSmall ? 0 : 7)
        .padding(.vertical, family == .systemSmall ? 0 : 4)
        .background {
            if family != .systemSmall {
                WidgetSoftLeafShape(smallRadius: 4, largeRadius: 14)
                    .fill(backgroundTopColor.opacity(0.58))
                    .overlay(
                        WidgetSoftLeafShape(smallRadius: 4, largeRadius: 14)
                            .stroke(checkboxBorderColor.opacity(0.52), lineWidth: 1)
                    )
            }
        }
    }
}

@available(iOSApplicationExtension 17.0, *)
struct TicklyWidget: Widget {
    let kind: String = "TicklyWidget"

    var body: some WidgetConfiguration {
        AppIntentConfiguration(
            kind: kind,
            intent: CategoryWidgetConfigurationIntent.self,
            provider: TicklyTimelineProvider()
        ) { entry in
            TicklyWidgetEntryView(entry: entry)
        }
        .configurationDisplayName("Tickly Category")
        .description("Show and check tasks in one category.")
        .supportedFamilies([.systemSmall, .systemMedium, .systemLarge])
    }
}
