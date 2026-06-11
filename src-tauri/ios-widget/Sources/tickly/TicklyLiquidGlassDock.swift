import Foundation
import SwiftUI
import UIKit
import WebKit

private struct TicklyLiquidGlassDockRequest: Decodable {
    let visible: Bool
    let streakLabel: String
    let graphLabel: String
    let archiveLabel: String
    let settingsLabel: String
    let streakEnabled: Bool
    let graphEnabled: Bool
    let archiveEnabled: Bool
    let settingsEnabled: Bool
}

private struct TicklyLiquidGlassDockAction: Encodable {
    let actionId: String
}

@_cdecl("tickly_configure_liquid_glass_dock")
public func ticklyConfigureLiquidGlassDock(
    _ webViewPointer: UnsafeMutableRawPointer?,
    _ viewControllerPointer: UnsafeMutableRawPointer?,
    _ requestJsonPointer: UnsafePointer<CChar>?
) -> Bool {
    guard
        let webViewPointer,
        let viewControllerPointer,
        let requestJsonPointer
    else {
        return false
    }

    let webView = Unmanaged<WKWebView>
        .fromOpaque(UnsafeRawPointer(webViewPointer))
        .takeUnretainedValue()
    let viewController = Unmanaged<UIViewController>
        .fromOpaque(UnsafeRawPointer(viewControllerPointer))
        .takeUnretainedValue()
    let requestJson = String(cString: requestJsonPointer)

    guard
        let requestData = requestJson.data(using: .utf8),
        let request = try? JSONDecoder().decode(TicklyLiquidGlassDockRequest.self, from: requestData)
    else {
        return false
    }

    if Thread.isMainThread {
        return TicklyLiquidGlassDockCoordinator.configure(
            request: request,
            webView: webView,
            viewController: viewController
        )
    }

    DispatchQueue.main.async {
        _ = TicklyLiquidGlassDockCoordinator.configure(
            request: request,
            webView: webView,
            viewController: viewController
        )
    }

    return true
}

private enum TicklyLiquidGlassDockCoordinator {
    private static let dockTag = 0x746c_6764

    static func configure(
        request: TicklyLiquidGlassDockRequest,
        webView: WKWebView,
        viewController: UIViewController
    ) -> Bool {
        guard let hostView = viewController.view else {
            return false
        }

        if !request.visible {
            (hostView.viewWithTag(dockTag) as? TicklyLiquidGlassDockView)?.removeFromSuperview()
            return true
        }

        let dockView: TicklyLiquidGlassDockView
        if let existingDock = hostView.viewWithTag(dockTag) as? TicklyLiquidGlassDockView {
            dockView = existingDock
            dockView.updateWebView(webView)
            dockView.updateParentViewController(viewController)
        } else {
            dockView = TicklyLiquidGlassDockView(
                webView: webView,
                parentViewController: viewController
            )
            dockView.tag = dockTag
            dockView.translatesAutoresizingMaskIntoConstraints = false
            hostView.addSubview(dockView)

            dockView.activateLayout(in: hostView)
        }

        dockView.update(request: request)
        hostView.bringSubviewToFront(dockView)
        return true
    }
}

private final class TicklyLiquidGlassDockView: UIView {
    private enum Style {
        static let ink = UIColor(red: 91 / 255, green: 88 / 255, blue: 82 / 255, alpha: 1)
    }

    private weak var webView: WKWebView?
    private weak var parentViewController: UIViewController?
    private var glassHostingController: UIViewController?
    private var fallbackToolbar: UIToolbar?
    private let streakItem = UIBarButtonItem(
        image: UIImage(systemName: "flame.fill"),
        style: .plain,
        target: nil,
        action: nil
    )
    private let graphItem = UIBarButtonItem(
        image: UIImage(systemName: "point.3.connected.trianglepath.dotted")
            ?? UIImage(systemName: "chart.line.uptrend.xyaxis"),
        style: .plain,
        target: nil,
        action: nil
    )
    private let archiveItem = UIBarButtonItem(
        image: UIImage(systemName: "archivebox.fill"),
        style: .plain,
        target: nil,
        action: nil
    )
    private let settingsItem = UIBarButtonItem(
        image: UIImage(systemName: "gearshape.fill"),
        style: .plain,
        target: nil,
        action: nil
    )
    private var requestedVisible = true
    private var keyboardHidden = false

    init(webView: WKWebView, parentViewController: UIViewController? = nil) {
        self.webView = webView
        self.parentViewController = parentViewController

        super.init(frame: .zero)

        setupView()
        setupKeyboardObservers()
    }

    @available(*, unavailable)
    required init?(coder: NSCoder) {
        nil
    }

    deinit {
        NotificationCenter.default.removeObserver(self)
        removeGlassHostingController()
    }

    func updateWebView(_ webView: WKWebView) {
        self.webView = webView
    }

    func updateParentViewController(_ viewController: UIViewController) {
        parentViewController = viewController
    }

    func activateLayout(in hostView: UIView) {
        if #available(iOS 26.0, *) {
            NSLayoutConstraint.activate([
                centerXAnchor.constraint(equalTo: hostView.centerXAnchor),
                bottomAnchor.constraint(
                    equalTo: hostView.safeAreaLayoutGuide.bottomAnchor,
                    constant: 26
                ),
                leadingAnchor.constraint(greaterThanOrEqualTo: hostView.leadingAnchor, constant: 18),
                trailingAnchor.constraint(lessThanOrEqualTo: hostView.trailingAnchor, constant: -18)
            ])
        } else {
            NSLayoutConstraint.activate([
                centerXAnchor.constraint(equalTo: hostView.centerXAnchor),
                bottomAnchor.constraint(
                    equalTo: hostView.safeAreaLayoutGuide.bottomAnchor,
                    constant: -10
                ),
                leadingAnchor.constraint(greaterThanOrEqualTo: hostView.leadingAnchor, constant: 18),
                trailingAnchor.constraint(lessThanOrEqualTo: hostView.trailingAnchor, constant: -18),
                heightAnchor.constraint(equalToConstant: 56),
                widthAnchor.constraint(equalToConstant: 244)
            ])
        }
    }

    func update(request: TicklyLiquidGlassDockRequest) {
        requestedVisible = request.visible
        if #available(iOS 26.0, *) {
            updateGlassDock(request: request)
        } else {
            updateFallbackToolbar(request: request)
        }
        updateVisibility(animated: true)
    }

    private func setupView() {
        isAccessibilityElement = false
        backgroundColor = .clear

        if #available(iOS 26.0, *) {
            setupGlassDock()
        } else {
            setupFallbackToolbar()
        }
    }

    @available(iOS 26.0, *)
    private func setupGlassDock() {
        updateGlassDock(
            request: TicklyLiquidGlassDockRequest(
                visible: true,
                streakLabel: "Streak",
                graphLabel: "Graph",
                archiveLabel: "Archive",
                settingsLabel: "Settings",
                streakEnabled: true,
                graphEnabled: true,
                archiveEnabled: true,
                settingsEnabled: true
            )
        )
    }

    @available(iOS 26.0, *)
    private func updateGlassDock(request: TicklyLiquidGlassDockRequest) {
        let content = TicklyGlassDockContent(
            streakLabel: request.streakLabel,
            graphLabel: request.graphLabel,
            archiveLabel: request.archiveLabel,
            settingsLabel: request.settingsLabel,
            streakEnabled: request.streakEnabled,
            graphEnabled: request.graphEnabled,
            archiveEnabled: request.archiveEnabled,
            settingsEnabled: request.settingsEnabled,
            performAction: { [weak self] actionId in
                self?.performAction(actionId)
            }
        )

        if let hostingController = glassHostingController as? UIHostingController<TicklyGlassDockContent> {
            hostingController.rootView = content
            return
        }

        let hostingController = UIHostingController(rootView: content)
        hostingController.view.backgroundColor = .clear
        hostingController.view.translatesAutoresizingMaskIntoConstraints = false
        glassHostingController = hostingController

        if let parentViewController {
            parentViewController.addChild(hostingController)
        }
        addSubview(hostingController.view)
        NSLayoutConstraint.activate([
            hostingController.view.topAnchor.constraint(equalTo: topAnchor),
            hostingController.view.leadingAnchor.constraint(equalTo: leadingAnchor),
            hostingController.view.trailingAnchor.constraint(equalTo: trailingAnchor),
            hostingController.view.bottomAnchor.constraint(equalTo: bottomAnchor)
        ])
        if parentViewController != nil {
            hostingController.didMove(toParent: parentViewController)
        }
    }

    private func removeGlassHostingController() {
        guard let hostingController = glassHostingController else {
            return
        }

        hostingController.willMove(toParent: nil)
        hostingController.view.removeFromSuperview()
        hostingController.removeFromParent()
        glassHostingController = nil
    }

    private func setupFallbackToolbar() {
        let toolbar = UIToolbar()
        fallbackToolbar = toolbar
        toolbar.translatesAutoresizingMaskIntoConstraints = false
        toolbar.delegate = self
        toolbar.tintColor = Style.ink
        toolbar.isTranslucent = true
        toolbar.clipsToBounds = false

        let appearance = UIToolbarAppearance()
        appearance.configureWithTransparentBackground()
        appearance.backgroundEffect = UIBlurEffect(style: .systemUltraThinMaterial)
        appearance.shadowColor = .clear
        toolbar.standardAppearance = appearance
        toolbar.compactAppearance = appearance
        if #available(iOS 15.0, *) {
            toolbar.scrollEdgeAppearance = appearance
            toolbar.compactScrollEdgeAppearance = appearance
        }

        addSubview(toolbar)
        toolbar.setItems(makeToolbarItems(), animated: false)

        NSLayoutConstraint.activate([
            toolbar.topAnchor.constraint(equalTo: topAnchor),
            toolbar.leadingAnchor.constraint(equalTo: leadingAnchor),
            toolbar.trailingAnchor.constraint(equalTo: trailingAnchor),
            toolbar.bottomAnchor.constraint(equalTo: bottomAnchor)
        ])
    }

    private func updateFallbackToolbar(request: TicklyLiquidGlassDockRequest) {
        configureItem(
            streakItem,
            title: request.streakLabel,
            actionId: "streak",
            enabled: request.streakEnabled
        )
        configureItem(
            graphItem,
            title: request.graphLabel,
            actionId: "graph",
            enabled: request.graphEnabled
        )
        configureItem(
            archiveItem,
            title: request.archiveLabel,
            actionId: "archive",
            enabled: request.archiveEnabled
        )
        configureItem(
            settingsItem,
            title: request.settingsLabel,
            actionId: "settings",
            enabled: request.settingsEnabled
        )
        fallbackToolbar?.setItems(makeToolbarItems(), animated: false)
    }

    private func makeToolbarItems() -> [UIBarButtonItem] {
        let leadingSpace = UIBarButtonItem(barButtonSystemItem: .flexibleSpace, target: nil, action: nil)
        let actionSpaceA = UIBarButtonItem(barButtonSystemItem: .fixedSpace, target: nil, action: nil)
        actionSpaceA.width = 12
        let actionSpaceB = UIBarButtonItem(barButtonSystemItem: .fixedSpace, target: nil, action: nil)
        actionSpaceB.width = 12
        let groupSpace = UIBarButtonItem(barButtonSystemItem: .fixedSpace, target: nil, action: nil)
        groupSpace.width = 30
        let trailingSpace = UIBarButtonItem(barButtonSystemItem: .flexibleSpace, target: nil, action: nil)
        return [
            leadingSpace,
            streakItem,
            actionSpaceA,
            graphItem,
            actionSpaceB,
            archiveItem,
            groupSpace,
            settingsItem,
            trailingSpace
        ]
    }

    private func configureItem(
        _ item: UIBarButtonItem,
        title: String,
        actionId: String,
        enabled: Bool
    ) {
        item.target = self
        switch actionId {
        case "streak":
            item.action = #selector(handleStreakAction)
        case "graph":
            item.action = #selector(handleGraphAction)
        case "archive":
            item.action = #selector(handleArchiveAction)
        default:
            item.action = #selector(handleSettingsAction)
        }
        item.isEnabled = enabled
        item.accessibilityLabel = title
        item.accessibilityIdentifier = "tickly.nativeDock.\(actionId)"
    }

    @objc private func handleStreakAction() {
        performAction("streak")
    }

    @objc private func handleGraphAction() {
        performAction("graph")
    }

    @objc private func handleArchiveAction() {
        performAction("archive")
    }

    @objc private func handleSettingsAction() {
        performAction("settings")
    }

    private func performAction(_ actionId: String) {
        UIImpactFeedbackGenerator(style: .light).impactOccurred()

        let action = TicklyLiquidGlassDockAction(actionId: actionId)
        guard
            let data = try? JSONEncoder().encode(action),
            let json = String(data: data, encoding: .utf8)
        else {
            return
        }

        let script = "window.dispatchEvent(new CustomEvent('tickly:nativeDockAction',{detail:\(json)}));"
        webView?.evaluateJavaScript(script)
    }

    private func setupKeyboardObservers() {
        NotificationCenter.default.addObserver(
            self,
            selector: #selector(keyboardWillChangeFrame(_:)),
            name: UIResponder.keyboardWillChangeFrameNotification,
            object: nil
        )
        NotificationCenter.default.addObserver(
            self,
            selector: #selector(keyboardWillHide(_:)),
            name: UIResponder.keyboardWillHideNotification,
            object: nil
        )
    }

    @objc private func keyboardWillChangeFrame(_ notification: Notification) {
        guard
            let hostView = superview,
            let endFrame = notification.userInfo?[UIResponder.keyboardFrameEndUserInfoKey] as? CGRect
        else {
            return
        }

        let convertedFrame = hostView.convert(endFrame, from: nil)
        keyboardHidden = convertedFrame.minY < hostView.bounds.maxY - 24
        updateVisibility(animated: true)
    }

    @objc private func keyboardWillHide(_ notification: Notification) {
        keyboardHidden = false
        updateVisibility(animated: true)
    }

    private func updateVisibility(animated: Bool) {
        let shouldShow = requestedVisible && !keyboardHidden
        let changes = {
            self.alpha = shouldShow ? 1 : 0
            self.transform = shouldShow ? .identity : CGAffineTransform(translationX: 0, y: 18)
        }

        isUserInteractionEnabled = shouldShow
        if animated {
            UIView.animate(
                withDuration: 0.22,
                delay: 0,
                options: [.beginFromCurrentState, .allowUserInteraction, .curveEaseOut],
                animations: changes
            )
        } else {
            changes()
        }
    }
}

extension TicklyLiquidGlassDockView: UIToolbarDelegate {
    func position(for bar: UIBarPositioning) -> UIBarPosition {
        .bottom
    }
}

@available(iOS 26.0, *)
private enum TicklyGlassDockButtonTreatment {
    case plain
}

@available(iOS 26.0, *)
private enum TicklyGlassDockStyle {
    static let buttonTreatment: TicklyGlassDockButtonTreatment = .plain
    static let containerGlass: Glass = .clear
    static let inkTreatment = Color(red: 91 / 255, green: 88 / 255, blue: 82 / 255)
    static let groupSpacing: CGFloat = 64
    static let featureButtonSpacing: CGFloat = 18
    static let iconSize: CGFloat = 20
    static let buttonSize: CGFloat = 44
    static let surfaceInset: CGFloat = 7
    static let surfaceUnderlayOpacity = 0.09
    static let surfaceVeilOpacity = 0.0
    static let surfaceStrokeOpacity = 0.14
    static let surfaceStrokeWidth = 0.75
    static let surfaceShadowOpacity = 0.10
}

@available(iOS 26.0, *)
private struct TicklyGlassDockContent: View {
    let streakLabel: String
    let graphLabel: String
    let archiveLabel: String
    let settingsLabel: String
    let streakEnabled: Bool
    let graphEnabled: Bool
    let archiveEnabled: Bool
    let settingsEnabled: Bool
    let performAction: (String) -> Void

    var body: some View {
        GlassEffectContainer(spacing: TicklyGlassDockStyle.groupSpacing) {
            HStack(spacing: TicklyGlassDockStyle.groupSpacing) {
                HStack(spacing: TicklyGlassDockStyle.featureButtonSpacing) {
                    dockButton(
                        actionId: "streak",
                        label: streakLabel,
                        systemImageName: "flame.fill",
                        isEnabled: streakEnabled
                    )
                    dockButton(
                        actionId: "graph",
                        label: graphLabel,
                        systemImageName: "graph.connected",
                        isEnabled: graphEnabled
                    )
                    dockButton(
                        actionId: "archive",
                        label: archiveLabel,
                        systemImageName: "archivebox.fill",
                        isEnabled: archiveEnabled
                    )
                }
                .padding(.horizontal, 12)
                .padding(.vertical, TicklyGlassDockStyle.surfaceInset)
                .background(
                    TicklyGlassDockStyle.inkTreatment.opacity(TicklyGlassDockStyle.surfaceUnderlayOpacity),
                    in: Capsule()
                )
                .overlay {
                    Capsule()
                        .stroke(
                            TicklyGlassDockStyle.inkTreatment.opacity(TicklyGlassDockStyle.surfaceStrokeOpacity),
                            lineWidth: TicklyGlassDockStyle.surfaceStrokeWidth
                        )
                }
                .shadow(
                    color: TicklyGlassDockStyle.inkTreatment.opacity(TicklyGlassDockStyle.surfaceShadowOpacity),
                    radius: 12,
                    x: 0,
                    y: 4
                )
                .glassEffect(TicklyGlassDockStyle.containerGlass.interactive(), in: Capsule())
                .overlay {
                    Capsule()
                        .fill(TicklyGlassDockStyle.inkTreatment.opacity(TicklyGlassDockStyle.surfaceVeilOpacity))
                        .allowsHitTesting(false)
                }

                dockButton(
                    actionId: "settings",
                    label: settingsLabel,
                    systemImageName: "gearshape.fill",
                    isEnabled: settingsEnabled
                )
                .padding(TicklyGlassDockStyle.surfaceInset)
                .background(
                    TicklyGlassDockStyle.inkTreatment.opacity(TicklyGlassDockStyle.surfaceUnderlayOpacity),
                    in: Circle()
                )
                .overlay {
                    Circle()
                        .stroke(
                            TicklyGlassDockStyle.inkTreatment.opacity(TicklyGlassDockStyle.surfaceStrokeOpacity),
                            lineWidth: TicklyGlassDockStyle.surfaceStrokeWidth
                        )
                }
                .shadow(
                    color: TicklyGlassDockStyle.inkTreatment.opacity(TicklyGlassDockStyle.surfaceShadowOpacity),
                    radius: 10,
                    x: 0,
                    y: 4
                )
                .glassEffect(TicklyGlassDockStyle.containerGlass.interactive(), in: Circle())
                .overlay {
                    Circle()
                        .fill(TicklyGlassDockStyle.inkTreatment.opacity(TicklyGlassDockStyle.surfaceVeilOpacity))
                        .allowsHitTesting(false)
                }
            }
        }
        .fixedSize()
        .accessibilityElement(children: .contain)
    }

    @ViewBuilder
    private func dockButton(
        actionId: String,
        label: String,
        systemImageName: String,
        isEnabled: Bool
    ) -> some View {
        switch TicklyGlassDockStyle.buttonTreatment {
        case .plain:
            Button {
                performAction(actionId)
            } label: {
                dockIcon(systemImageName)
            }
            .buttonStyle(.plain)
            .disabled(!isEnabled)
            .accessibilityLabel(Text(label))
            .accessibilityIdentifier("tickly.nativeDock.\(actionId)")
        }
    }

    private func dockIcon(_ systemImageName: String) -> some View {
        Group {
            if systemImageName == "graph.connected" {
                TicklyGraphDockIcon()
            } else {
                Image(systemName: systemImageName)
                    .font(.system(size: TicklyGlassDockStyle.iconSize, weight: .medium))
            }
        }
        .frame(
            width: TicklyGlassDockStyle.buttonSize,
            height: TicklyGlassDockStyle.buttonSize
        )
        .foregroundStyle(TicklyGlassDockStyle.inkTreatment)
    }
}

@available(iOS 26.0, *)
private struct TicklyGraphDockIcon: View {
    var body: some View {
        ZStack {
            Path { path in
                path.move(to: CGPoint(x: 4.2, y: 5.2))
                path.addLine(to: CGPoint(x: 15.8, y: 5.2))
                path.move(to: CGPoint(x: 4.2, y: 5.2))
                path.addLine(to: CGPoint(x: 10, y: 15.8))
                path.move(to: CGPoint(x: 15.8, y: 5.2))
                path.addLine(to: CGPoint(x: 10, y: 15.8))
            }
            .stroke(
                TicklyGlassDockStyle.inkTreatment,
                style: StrokeStyle(lineWidth: 2, lineCap: .round, lineJoin: .round)
            )

            node(at: CGPoint(x: 4.2, y: 5.2))
            node(at: CGPoint(x: 15.8, y: 5.2))
            node(at: CGPoint(x: 10, y: 15.8))
        }
        .frame(width: 20, height: 20)
    }

    private func node(at point: CGPoint) -> some View {
        Circle()
            .stroke(TicklyGlassDockStyle.inkTreatment, lineWidth: 2)
            .frame(width: 5.8, height: 5.8)
            .position(point)
    }
}
