import SwiftUI
import WidgetKit

@available(iOSApplicationExtension 17.0, *)
struct TicklyLockScreenWidgetEntryView: View {
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

    private func completedCount(for category: WidgetCategorySummary) -> Int {
        max(0, category.totalCount - category.pendingCount)
    }

    private func progressValue(for category: WidgetCategorySummary) -> Double {
        guard category.totalCount > 0 else {
            return 1
        }

        return Double(completedCount(for: category)) / Double(category.totalCount)
    }

    var body: some View {
        Group {
            if let selectedCategory {
                switch family {
                case .accessoryInline:
                    inlineView(category: selectedCategory)
                case .accessoryCircular:
                    circularView(category: selectedCategory)
                default:
                    rectangularView(category: selectedCategory)
                }
            } else {
                switch family {
                case .accessoryInline:
                    Text(WidgetCopy.allDone)
                case .accessoryCircular:
                    Image(systemName: "checkmark.circle.fill")
                default:
                    Text(WidgetCopy.allDone)
                        .font(.caption)
                }
            }
        }
        .containerBackground(for: .widget) {
            Color.clear
        }
    }

    @ViewBuilder
    private func inlineView(category: WidgetCategorySummary) -> some View {
        let visibleItems = Array(category.pendingItems.prefix(1))
        let overflowCount = max(0, category.pendingItems.count - visibleItems.count)

        if let firstPending = visibleItems.first {
            HStack(spacing: 4) {
                Image(systemName: "square")
                Text(firstPending.text)

                if overflowCount > 0 {
                    Text("+\(overflowCount)")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }
            }
        } else {
            Text(WidgetCopy.allDone)
        }
    }

    @ViewBuilder
    private func circularView(category: WidgetCategorySummary) -> some View {
        let progress = progressValue(for: category)
        let progressPercent = Int((progress * 100).rounded())
        let progressColor: Color = progress >= 1 ? .green : .accentColor

        ZStack {
            Circle()
                .stroke(progressColor.opacity(0.24), lineWidth: 4.5)

            Circle()
                .trim(from: 0, to: progress)
                .stroke(
                    progressColor,
                    style: StrokeStyle(lineWidth: 4.5, lineCap: .round, lineJoin: .round)
                )
                .rotationEffect(.degrees(-90))

            VStack(spacing: 2) {
                Text(category.categoryName)
                    .font(.system(size: 8, weight: .semibold))
                    .lineLimit(1)
                    .minimumScaleFactor(0.45)

                Text("\(progressPercent)%")
                    .font(.system(size: 9, weight: .medium))
                    .minimumScaleFactor(0.75)
            }
        }
        .padding(2)
    }

    @ViewBuilder
    private func rectangularView(category: WidgetCategorySummary) -> some View {
        let visibleItems = Array(category.pendingItems.prefix(2))
        let overflowCount = max(0, category.pendingItems.count - visibleItems.count)

        VStack(alignment: .leading, spacing: 3) {
            Text(category.categoryName)
                .font(.caption2.weight(.semibold))
                .lineLimit(1)
                .minimumScaleFactor(0.75)

            if visibleItems.isEmpty {
                Text(WidgetCopy.allDone)
                    .font(.caption2)
            } else {
                ForEach(visibleItems) { item in
                    pendingItemRow(item: item)
                }

                if overflowCount > 0 {
                    HStack {
                        Spacer(minLength: 0)
                        Text(WidgetCopy.more(overflowCount))
                            .font(.caption2)
                            .foregroundStyle(.secondary)
                    }
                }
            }
        }
    }

    @ViewBuilder
    private func pendingItemRow(item: WidgetCategoryPendingItem) -> some View {
        HStack(spacing: 6) {
            Image(systemName: "square")
                .imageScale(.small)
                .accessibilityHidden(true)

            Text(item.text)
                .font(.caption)
                .lineLimit(1)
                .frame(maxWidth: .infinity, alignment: .leading)
        }
    }
}

@available(iOSApplicationExtension 17.0, *)
struct TicklyLockScreenWidget: Widget {
    let kind: String = "TicklyLockScreenWidget"

    var body: some WidgetConfiguration {
        AppIntentConfiguration(
            kind: kind,
            intent: CategoryWidgetConfigurationIntent.self,
            provider: TicklyTimelineProvider()
        ) { entry in
            TicklyLockScreenWidgetEntryView(entry: entry)
        }
        .configurationDisplayName("Tickly Lock Screen")
        .description("Show checklist items on the Lock Screen.")
        .supportedFamilies([.accessoryInline, .accessoryCircular, .accessoryRectangular])
    }
}
