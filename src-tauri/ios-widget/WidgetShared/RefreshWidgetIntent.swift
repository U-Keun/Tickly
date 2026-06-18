import AppIntents
import WidgetKit

@available(iOS 17.0, iOSApplicationExtension 17.0, *)
struct RefreshWidgetIntent: AppIntent {
    static var title: LocalizedStringResource = "Refresh Widget"
    static var description = IntentDescription("Reload the widget timeline.")
    static var openAppWhenRun = false

    func perform() async throws -> some IntentResult {
        WidgetCenter.shared.reloadAllTimelines()
        return .result()
    }
}
