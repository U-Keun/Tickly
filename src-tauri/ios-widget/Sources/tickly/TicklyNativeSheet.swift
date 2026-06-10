import Foundation
import UIKit
import WebKit

private struct TicklyNativeSheetRequest: Decodable {
    let token: String
    let kind: String
    let title: String
    let message: String?
    let text: TicklyNativeSheetTextRequest?
    let form: TicklyNativeSheetFormRequest?
    let actions: [TicklyNativeSheetActionRequest]?
    let cancelLabel: String
}

private struct TicklyNativeSheetTextRequest: Decodable {
    let label: String
    let placeholder: String
    let initialValue: String
    let confirmLabel: String
}

private struct TicklyNativeSheetFormRequest: Decodable {
    let fields: [TicklyNativeSheetFormFieldRequest]
    let confirmLabel: String
}

private struct TicklyNativeSheetFormFieldRequest: Decodable {
    let id: String
    let kind: String
    let label: String
    let placeholder: String
    let initialValue: String
    let initialTags: [String]?
    let suggestions: [String]?
    let required: Bool?
}

private struct TicklyNativeSheetActionRequest: Decodable {
    let id: String
    let label: String
    let tone: String?
    let disabled: Bool?
}

private struct TicklyNativeSheetResult: Encodable {
    let token: String
    let status: String
    let value: String?
    let values: [String: TicklyNativeSheetValue]?
    let actionId: String?
}

private enum TicklyNativeSheetValue: Encodable {
    case string(String)
    case strings([String])

    func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        switch self {
        case .string(let value):
            try container.encode(value)
        case .strings(let values):
            try container.encode(values)
        }
    }
}

@_cdecl("tickly_show_native_sheet")
public func ticklyShowNativeSheet(
    _ webViewPointer: UnsafeMutableRawPointer?,
    _ viewControllerPointer: UnsafeMutableRawPointer?,
    _ requestJsonPointer: UnsafePointer<CChar>?
) -> Bool {
    guard #available(iOS 15.0, *) else {
        return false
    }

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
    let presentingViewController = Unmanaged<UIViewController>
        .fromOpaque(UnsafeRawPointer(viewControllerPointer))
        .takeUnretainedValue()
    let requestJson = String(cString: requestJsonPointer)

    guard let requestData = requestJson.data(using: .utf8),
          let request = try? JSONDecoder().decode(TicklyNativeSheetRequest.self, from: requestData),
          ticklyCanRenderNativeSheet(request)
    else {
        return false
    }

    if Thread.isMainThread {
        return ticklyPresentNativeSheet(
            request: request,
            webView: webView,
            presentingViewController: presentingViewController
        )
    }

    DispatchQueue.main.async {
        _ = ticklyPresentNativeSheet(
            request: request,
            webView: webView,
            presentingViewController: presentingViewController
        )
    }
    return true
}

private func ticklyCanRenderNativeSheet(_ request: TicklyNativeSheetRequest) -> Bool {
    if request.kind == "text" {
        return request.text != nil
    }

    if request.kind == "form" {
        return request.form?.fields.isEmpty == false
    }

    if request.kind == "actions" {
        return request.actions != nil
    }

    return false
}

@available(iOS 15.0, *)
private func ticklyPresentNativeSheet(
    request: TicklyNativeSheetRequest,
    webView: WKWebView,
    presentingViewController: UIViewController
) -> Bool {
    let topViewController = ticklyTopViewController(from: presentingViewController)
    guard !(topViewController is TicklyNativeSheetViewController) else {
        return false
    }

    let sheetViewController = TicklyNativeSheetViewController(
        request: request,
        webView: webView
    )
    let preferredHeight = ticklyPreferredSheetHeight(for: request)
    sheetViewController.configureLeafPresentation(preferredHeight: preferredHeight)
    topViewController.present(sheetViewController, animated: true)
    return true
}

private func ticklyPreferredSheetHeight(for request: TicklyNativeSheetRequest) -> CGFloat {
    if request.kind == "text" {
        return 292
    }

    if request.kind == "form" {
        let fieldCount = CGFloat(request.form?.fields.count ?? 0)
        return min(max(292 + fieldCount * 92, 390), 560)
    }

    let actionCount = CGFloat(request.actions?.count ?? 0)
    let messageHeight: CGFloat = (request.message?.isEmpty == false) ? 28 : 0
    return min(max(236 + messageHeight + actionCount * 58, 300), 430)
}

private func ticklyLeafSheetPath(
    in bounds: CGRect,
    majorRadius: CGFloat = 24,
    minorRadius: CGFloat = 6
) -> CGPath {
    let maxRadius = min(bounds.width, bounds.height) / 2
    let majorRadius = min(max(0, majorRadius), maxRadius)
    let minorRadius = min(max(0, minorRadius), maxRadius)

    let path = UIBezierPath()
    path.move(to: CGPoint(x: bounds.minX + minorRadius, y: bounds.minY))
    path.addLine(to: CGPoint(x: bounds.maxX - majorRadius, y: bounds.minY))
    path.addQuadCurve(
        to: CGPoint(x: bounds.maxX, y: bounds.minY + majorRadius),
        controlPoint: CGPoint(x: bounds.maxX, y: bounds.minY)
    )
    path.addLine(to: CGPoint(x: bounds.maxX, y: bounds.maxY - minorRadius))
    path.addQuadCurve(
        to: CGPoint(x: bounds.maxX - minorRadius, y: bounds.maxY),
        controlPoint: CGPoint(x: bounds.maxX, y: bounds.maxY)
    )
    path.addLine(to: CGPoint(x: bounds.minX + majorRadius, y: bounds.maxY))
    path.addQuadCurve(
        to: CGPoint(x: bounds.minX, y: bounds.maxY - majorRadius),
        controlPoint: CGPoint(x: bounds.minX, y: bounds.maxY)
    )
    path.addLine(to: CGPoint(x: bounds.minX, y: bounds.minY + minorRadius))
    path.addQuadCurve(
        to: CGPoint(x: bounds.minX + minorRadius, y: bounds.minY),
        controlPoint: CGPoint(x: bounds.minX, y: bounds.minY)
    )
    path.close()

    return path.cgPath
}

@available(iOS 15.0, *)
private func ticklyTopViewController(from viewController: UIViewController) -> UIViewController {
    var current = viewController
    while let presented = current.presentedViewController {
        current = presented
    }
    return current
}

@available(iOS 15.0, *)
private final class TicklyNativeSheetViewController: UIViewController, UIAdaptivePresentationControllerDelegate, UITextFieldDelegate, UITextViewDelegate {
    fileprivate enum Style {
        static let ink = UIColor(red: 91 / 255, green: 88 / 255, blue: 82 / 255, alpha: 1)
        static let inkMuted = UIColor(red: 122 / 255, green: 119 / 255, blue: 111 / 255, alpha: 1)
        static let paper = UIColor(red: 248 / 255, green: 247 / 255, blue: 243 / 255, alpha: 1)
        static let canvas = UIColor(red: 242 / 255, green: 239 / 255, blue: 232 / 255, alpha: 1)
        static let accentSky = UIColor(red: 168 / 255, green: 189 / 255, blue: 219 / 255, alpha: 1)
        static let accentSkyStrong = UIColor(red: 142 / 255, green: 169 / 255, blue: 207 / 255, alpha: 1)
        static let accentPeach = UIColor(red: 229 / 255, green: 185 / 255, blue: 160 / 255, alpha: 1)
        static let accentPeachStrong = UIColor(red: 215 / 255, green: 164 / 255, blue: 138 / 255, alpha: 1)
    }

    private let request: TicklyNativeSheetRequest
    private weak var webView: WKWebView?
    private let textField = UITextField()
    private let saveButton = UIButton(type: .system)
    private let surfaceView = UIView()
    private let surfaceFillLayer = CAShapeLayer()
    private let surfaceBorderLayer = CAShapeLayer()
    private var formTextFields: [String: UITextField] = [:]
    private var formTextViews: [String: UITextView] = [:]
    private var formTagFields: [String: TagFieldView] = [:]
    private var formRequiredFieldIds = Set<String>()
    private var textViewFieldIds: [ObjectIdentifier: String] = [:]
    private var textViewPlaceholders: [ObjectIdentifier: String] = [:]
    private var leafTransitioningDelegate: TicklyLeafSheetTransitioningDelegate?
    private var didComplete = false

    init(request: TicklyNativeSheetRequest, webView: WKWebView) {
        self.request = request
        self.webView = webView
        super.init(nibName: nil, bundle: nil)
    }

    @available(*, unavailable)
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        view.backgroundColor = .clear
        buildLayout()
    }

    override func viewDidLayoutSubviews() {
        super.viewDidLayoutSubviews()

        updateSurfaceShape()
    }

    func configureLeafPresentation(preferredHeight: CGFloat) {
        let transitionDelegate = TicklyLeafSheetTransitioningDelegate()
        leafTransitioningDelegate = transitionDelegate
        modalPresentationStyle = .custom
        transitioningDelegate = transitionDelegate
        preferredContentSize = CGSize(width: 440, height: preferredHeight)
    }

    func presentationControllerDidDismiss(_ presentationController: UIPresentationController) {
        complete(status: "cancelled", value: nil, values: nil, actionId: nil, shouldDismiss: false)
    }

    fileprivate func requestCancelFromPresentation() {
        complete(status: "cancelled", value: nil, values: nil, actionId: nil, shouldDismiss: true)
    }

    func textFieldShouldReturn(_ textField: UITextField) -> Bool {
        if request.kind == "form" {
            saveForm()
        } else {
            saveText()
        }
        return true
    }

    func textViewDidBeginEditing(_ textView: UITextView) {
        let key = ObjectIdentifier(textView)
        guard let placeholder = textViewPlaceholders[key],
              textView.text == placeholder,
              textView.textColor != Style.ink
        else {
            return
        }

        textView.text = ""
        textView.textColor = Style.ink
    }

    func textViewDidEndEditing(_ textView: UITextView) {
        let key = ObjectIdentifier(textView)
        guard let placeholder = textViewPlaceholders[key],
              textView.text.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty
        else {
            return
        }

        textView.text = placeholder
        textView.textColor = Style.inkMuted.withAlphaComponent(0.55)
    }

    func textViewDidChange(_ textView: UITextView) {
        updateSaveButtonState()
    }

    private func updateSurfaceShape() {
        guard !surfaceView.bounds.isEmpty else {
            return
        }

        let bounds = surfaceView.bounds
        surfaceBorderLayer.frame = bounds
        surfaceBorderLayer.path = ticklyLeafSheetPath(in: bounds)

        let borderWidth: CGFloat = 2.5
        let fillBounds = bounds.insetBy(dx: borderWidth, dy: borderWidth)
        surfaceFillLayer.frame = bounds
        surfaceFillLayer.path = ticklyLeafSheetPath(
            in: fillBounds,
            majorRadius: 24 - borderWidth,
            minorRadius: 6 - borderWidth
        )

        view.layer.shadowPath = ticklyLeafSheetPath(in: view.bounds)
    }

    private func buildLayout() {
        surfaceView.translatesAutoresizingMaskIntoConstraints = false
        surfaceView.backgroundColor = .clear
        surfaceView.layer.addSublayer(surfaceBorderLayer)
        surfaceView.layer.addSublayer(surfaceFillLayer)
        surfaceFillLayer.fillColor = UIColor.white.cgColor
        surfaceBorderLayer.fillColor = Style.ink.cgColor

        view.layer.shadowColor = UIColor.black.cgColor
        view.layer.shadowOpacity = 0.14
        view.layer.shadowRadius = 18
        view.layer.shadowOffset = CGSize(width: 0, height: -4)
        view.addSubview(surfaceView)

        let scrollView = UIScrollView()
        scrollView.translatesAutoresizingMaskIntoConstraints = false
        scrollView.keyboardDismissMode = .interactive
        scrollView.alwaysBounceVertical = false
        surfaceView.addSubview(scrollView)

        let stack = UIStackView()
        stack.translatesAutoresizingMaskIntoConstraints = false
        stack.axis = .vertical
        stack.spacing = 16
        scrollView.addSubview(stack)

        addHeader(to: stack)

        if request.kind == "text", let textRequest = request.text {
            addTextContent(textRequest, to: stack)
            updateSaveButtonState()
        } else if request.kind == "form", let formRequest = request.form {
            addFormContent(formRequest, to: stack)
            updateSaveButtonState()
        } else {
            addActionContent(to: stack)
        }

        NSLayoutConstraint.activate([
            surfaceView.topAnchor.constraint(equalTo: view.topAnchor),
            surfaceView.leadingAnchor.constraint(equalTo: view.leadingAnchor),
            surfaceView.trailingAnchor.constraint(equalTo: view.trailingAnchor),
            surfaceView.bottomAnchor.constraint(equalTo: view.bottomAnchor),

            scrollView.topAnchor.constraint(equalTo: surfaceView.topAnchor),
            scrollView.leadingAnchor.constraint(equalTo: surfaceView.leadingAnchor),
            scrollView.trailingAnchor.constraint(equalTo: surfaceView.trailingAnchor),
            scrollView.bottomAnchor.constraint(equalTo: surfaceView.safeAreaLayoutGuide.bottomAnchor),

            stack.topAnchor.constraint(equalTo: scrollView.contentLayoutGuide.topAnchor, constant: 24),
            stack.leadingAnchor.constraint(equalTo: scrollView.contentLayoutGuide.leadingAnchor, constant: 24),
            stack.trailingAnchor.constraint(equalTo: scrollView.contentLayoutGuide.trailingAnchor, constant: -24),
            stack.bottomAnchor.constraint(equalTo: scrollView.contentLayoutGuide.bottomAnchor, constant: -24),
            stack.widthAnchor.constraint(equalTo: scrollView.frameLayoutGuide.widthAnchor, constant: -48)
        ])
    }

    private func addHeader(to stack: UIStackView) {
        let titleLabel = UILabel()
        titleLabel.text = request.title
        titleLabel.font = .preferredFont(forTextStyle: .title2)
        titleLabel.adjustsFontForContentSizeCategory = true
        titleLabel.textColor = Style.ink
        titleLabel.numberOfLines = 0
        stack.addArrangedSubview(titleLabel)

        guard let message = request.message, !message.isEmpty else {
            stack.setCustomSpacing(22, after: titleLabel)
            return
        }

        let messageLabel = UILabel()
        messageLabel.text = message
        messageLabel.font = .preferredFont(forTextStyle: .subheadline)
        messageLabel.adjustsFontForContentSizeCategory = true
        messageLabel.textColor = Style.inkMuted
        messageLabel.numberOfLines = 0
        stack.addArrangedSubview(messageLabel)
        stack.setCustomSpacing(22, after: messageLabel)
    }

    private func addTextContent(_ textRequest: TicklyNativeSheetTextRequest, to stack: UIStackView) {
        let fieldLabel = UILabel()
        fieldLabel.text = textRequest.label
        fieldLabel.font = .preferredFont(forTextStyle: .subheadline)
        fieldLabel.adjustsFontForContentSizeCategory = true
        fieldLabel.textColor = Style.ink

        textField.text = textRequest.initialValue
        textField.placeholder = textRequest.placeholder
        textField.font = .preferredFont(forTextStyle: .body)
        textField.adjustsFontForContentSizeCategory = true
        textField.textColor = Style.ink
        textField.tintColor = Style.accentSkyStrong
        textField.backgroundColor = Style.paper
        textField.layer.cornerRadius = 14
        textField.layer.borderColor = Style.ink.cgColor
        textField.layer.borderWidth = 2
        textField.returnKeyType = .done
        textField.clearButtonMode = .whileEditing
        textField.autocorrectionType = .default
        textField.delegate = self
        textField.addTarget(self, action: #selector(textDidChange), for: .editingChanged)
        textField.heightAnchor.constraint(greaterThanOrEqualToConstant: 52).isActive = true
        textField.leftView = UIView(frame: CGRect(x: 0, y: 0, width: 14, height: 1))
        textField.leftViewMode = .always
        textField.rightView = UIView(frame: CGRect(x: 0, y: 0, width: 14, height: 1))
        textField.rightViewMode = .always

        let buttonStack = UIStackView()
        buttonStack.axis = .horizontal
        buttonStack.spacing = 10
        buttonStack.distribution = .fillEqually

        saveButton.configuration = buttonConfiguration(
            title: textRequest.confirmLabel,
            backgroundColor: Style.accentSkyStrong,
            foregroundColor: Style.ink
        )
        saveButton.addTarget(self, action: #selector(saveButtonTapped), for: .touchUpInside)
        saveButton.heightAnchor.constraint(greaterThanOrEqualToConstant: 48).isActive = true

        let cancelButton = UIButton(type: .system)
        cancelButton.configuration = buttonConfiguration(
            title: request.cancelLabel,
            backgroundColor: Style.canvas,
            foregroundColor: Style.inkMuted
        )
        cancelButton.addTarget(self, action: #selector(cancelButtonTapped), for: .touchUpInside)
        cancelButton.heightAnchor.constraint(greaterThanOrEqualToConstant: 48).isActive = true

        buttonStack.addArrangedSubview(saveButton)
        buttonStack.addArrangedSubview(cancelButton)

        stack.addArrangedSubview(fieldLabel)
        stack.setCustomSpacing(8, after: fieldLabel)
        stack.addArrangedSubview(textField)
        stack.addArrangedSubview(buttonStack)
    }

    private func addFormContent(_ formRequest: TicklyNativeSheetFormRequest, to stack: UIStackView) {
        for field in formRequest.fields {
            if field.required ?? false {
                formRequiredFieldIds.insert(field.id)
            }

            if field.kind == "tags" {
                let tagField = TagFieldView(
                    label: field.label,
                    placeholder: field.placeholder,
                    initialTags: field.initialTags ?? [],
                    suggestions: field.suggestions ?? [],
                    style: Style.self
                )
                tagField.onChange = { [weak self] in
                    self?.updateSaveButtonState()
                }
                formTagFields[field.id] = tagField
                stack.addArrangedSubview(tagField)
            } else if field.kind == "textarea" {
                let textView = UITextView()
                textView.font = .preferredFont(forTextStyle: .body)
                textView.adjustsFontForContentSizeCategory = true
                textView.tintColor = Style.accentSkyStrong
                textView.backgroundColor = Style.paper
                textView.layer.cornerRadius = 14
                textView.layer.borderColor = Style.ink.cgColor
                textView.layer.borderWidth = 2
                textView.accessibilityLabel = field.label
                textView.keyboardDismissMode = .interactive
                textView.delegate = self
                textView.textContainerInset = UIEdgeInsets(top: 12, left: 10, bottom: 12, right: 10)
                textView.heightAnchor.constraint(greaterThanOrEqualToConstant: 112).isActive = true
                formTextViews[field.id] = textView
                textViewFieldIds[ObjectIdentifier(textView)] = field.id
                textViewPlaceholders[ObjectIdentifier(textView)] = field.placeholder

                if field.initialValue.isEmpty {
                    textView.text = field.placeholder
                    textView.textColor = Style.inkMuted.withAlphaComponent(0.55)
                } else {
                    textView.text = field.initialValue
                    textView.textColor = Style.ink
                }

                stack.addArrangedSubview(textView)
            } else {
                let fieldInput = UITextField()
                fieldInput.text = field.initialValue
                fieldInput.placeholder = field.placeholder
                fieldInput.font = .preferredFont(forTextStyle: .body)
                fieldInput.adjustsFontForContentSizeCategory = true
                fieldInput.textColor = Style.ink
                fieldInput.tintColor = Style.accentSkyStrong
                fieldInput.backgroundColor = Style.paper
                fieldInput.layer.cornerRadius = 14
                fieldInput.layer.borderColor = Style.ink.cgColor
                fieldInput.layer.borderWidth = 2
                fieldInput.accessibilityLabel = field.label
                fieldInput.returnKeyType = .done
                fieldInput.clearButtonMode = .whileEditing
                fieldInput.autocorrectionType = .default
                fieldInput.delegate = self
                fieldInput.addTarget(self, action: #selector(textDidChange), for: .editingChanged)
                fieldInput.heightAnchor.constraint(greaterThanOrEqualToConstant: 52).isActive = true
                fieldInput.leftView = UIView(frame: CGRect(x: 0, y: 0, width: 14, height: 1))
                fieldInput.leftViewMode = .always
                fieldInput.rightView = UIView(frame: CGRect(x: 0, y: 0, width: 14, height: 1))
                fieldInput.rightViewMode = .always
                formTextFields[field.id] = fieldInput
                stack.addArrangedSubview(fieldInput)
            }
        }

        let buttonStack = UIStackView()
        buttonStack.axis = .horizontal
        buttonStack.spacing = 10
        buttonStack.distribution = .fillEqually

        saveButton.configuration = buttonConfiguration(
            title: formRequest.confirmLabel,
            backgroundColor: Style.accentSkyStrong,
            foregroundColor: Style.ink
        )
        saveButton.addTarget(self, action: #selector(saveButtonTapped), for: .touchUpInside)
        saveButton.heightAnchor.constraint(greaterThanOrEqualToConstant: 48).isActive = true

        let cancelButton = UIButton(type: .system)
        cancelButton.configuration = buttonConfiguration(
            title: request.cancelLabel,
            backgroundColor: Style.canvas,
            foregroundColor: Style.inkMuted
        )
        cancelButton.addTarget(self, action: #selector(cancelButtonTapped), for: .touchUpInside)
        cancelButton.heightAnchor.constraint(greaterThanOrEqualToConstant: 48).isActive = true

        buttonStack.addArrangedSubview(saveButton)
        buttonStack.addArrangedSubview(cancelButton)
        stack.addArrangedSubview(buttonStack)
    }

    private func addActionContent(to stack: UIStackView) {
        let actions = request.actions ?? []

        for action in actions {
            let button = UIButton(type: .system)
            let style = actionStyle(for: action.tone)
            button.configuration = buttonConfiguration(
                title: action.label,
                backgroundColor: style.background,
                foregroundColor: style.foreground,
                imageName: imageName(for: action.id)
            )
            button.contentHorizontalAlignment = .leading
            button.accessibilityIdentifier = action.id
            button.isEnabled = !(action.disabled ?? false)
            button.alpha = button.isEnabled ? 1 : 0.45
            button.addTarget(self, action: #selector(actionButtonTapped(_:)), for: .touchUpInside)
            button.heightAnchor.constraint(greaterThanOrEqualToConstant: 48).isActive = true
            stack.addArrangedSubview(button)
        }

        if let lastActionView = stack.arrangedSubviews.last {
            stack.setCustomSpacing(10, after: lastActionView)
        }

        let cancelButton = UIButton(type: .system)
        cancelButton.configuration = buttonConfiguration(
            title: request.cancelLabel,
            backgroundColor: Style.canvas,
            foregroundColor: Style.inkMuted
        )
        cancelButton.addTarget(self, action: #selector(cancelButtonTapped), for: .touchUpInside)
        cancelButton.heightAnchor.constraint(greaterThanOrEqualToConstant: 48).isActive = true
        stack.addArrangedSubview(cancelButton)
    }

    private func buttonConfiguration(
        title: String,
        backgroundColor: UIColor,
        foregroundColor: UIColor,
        imageName: String? = nil
    ) -> UIButton.Configuration {
        var configuration = UIButton.Configuration.filled()
        configuration.title = title
        configuration.baseBackgroundColor = backgroundColor
        configuration.baseForegroundColor = foregroundColor
        configuration.cornerStyle = .medium
        configuration.contentInsets = NSDirectionalEdgeInsets(
            top: 12,
            leading: 16,
            bottom: 12,
            trailing: 16
        )
        configuration.imagePadding = 10

        if let imageName,
           let image = UIImage(systemName: imageName)
        {
            configuration.image = image
        }

        return configuration
    }

    private func actionStyle(for tone: String?) -> (background: UIColor, foreground: UIColor) {
        switch tone {
        case "primary":
            return (Style.accentSkyStrong, Style.ink)
        case "danger":
            return (Style.accentPeach, Style.ink)
        default:
            return (Style.paper, Style.ink)
        }
    }

    private func imageName(for actionId: String) -> String? {
        switch actionId {
        case "rename":
            return "pencil"
        case "editOrder":
            return "line.3.horizontal"
        case "delete":
            return "trash"
        default:
            return nil
        }
    }

    @objc private func textDidChange() {
        updateSaveButtonState()
    }

    @objc private func saveButtonTapped() {
        if request.kind == "form" {
            saveForm()
        } else {
            saveText()
        }
    }

    @objc private func cancelButtonTapped() {
        complete(status: "cancelled", value: nil, values: nil, actionId: nil, shouldDismiss: true)
    }

    @objc private func actionButtonTapped(_ sender: UIButton) {
        guard let actionId = sender.accessibilityIdentifier else {
            return
        }

        complete(status: "action", value: nil, values: nil, actionId: actionId, shouldDismiss: true)
    }

    private func saveText() {
        let value = (textField.text ?? "").trimmingCharacters(in: .whitespacesAndNewlines)
        guard !value.isEmpty else {
            updateSaveButtonState()
            return
        }

        complete(status: "saved", value: value, values: nil, actionId: nil, shouldDismiss: true)
    }

    private func saveForm() {
        guard formIsValid() else {
            updateSaveButtonState()
            return
        }

        var values: [String: TicklyNativeSheetValue] = [:]

        for (fieldId, fieldInput) in formTextFields {
            values[fieldId] = .string((fieldInput.text ?? "").trimmingCharacters(in: .whitespacesAndNewlines))
        }

        for (fieldId, textView) in formTextViews {
            values[fieldId] = .string(normalizedTextViewValue(textView))
        }

        for (fieldId, tagField) in formTagFields {
            values[fieldId] = .strings(tagField.tags)
        }

        complete(status: "saved", value: nil, values: values, actionId: nil, shouldDismiss: true)
    }

    private func updateSaveButtonState() {
        let isEnabled = request.kind == "form"
            ? formIsValid()
            : !(textField.text ?? "").trimmingCharacters(in: .whitespacesAndNewlines).isEmpty
        saveButton.isEnabled = isEnabled

        var configuration = saveButton.configuration
        configuration?.baseBackgroundColor = isEnabled
            ? Style.accentSkyStrong
            : Style.accentSky.withAlphaComponent(0.5)
        configuration?.baseForegroundColor = isEnabled
            ? Style.ink
            : Style.inkMuted.withAlphaComponent(0.6)
        saveButton.configuration = configuration
    }

    private func formIsValid() -> Bool {
        for fieldId in formRequiredFieldIds {
            if let fieldInput = formTextFields[fieldId] {
                if (fieldInput.text ?? "").trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
                    return false
                }
                continue
            }

            if let textView = formTextViews[fieldId] {
                if normalizedTextViewValue(textView).isEmpty {
                    return false
                }
                continue
            }

            if let tagField = formTagFields[fieldId] {
                if tagField.tags.isEmpty {
                    return false
                }
                continue
            }

            return false
        }

        return true
    }

    private func normalizedTextViewValue(_ textView: UITextView) -> String {
        let key = ObjectIdentifier(textView)
        if let placeholder = textViewPlaceholders[key],
           textView.text == placeholder,
           textView.textColor != Style.ink
        {
            return ""
        }

        return textView.text.trimmingCharacters(in: .whitespacesAndNewlines)
    }

    private func complete(
        status: String,
        value: String?,
        values: [String: TicklyNativeSheetValue]?,
        actionId: String?,
        shouldDismiss: Bool
    ) {
        guard !didComplete else {
            return
        }

        didComplete = true

        if shouldDismiss {
            dismiss(animated: true) { [weak self] in
                self?.emitResult(status: status, value: value, values: values, actionId: actionId)
            }
            return
        }

        emitResult(status: status, value: value, values: values, actionId: actionId)
    }

    private final class TagFieldView: UIView, UITextFieldDelegate {
        var onChange: (() -> Void)?

        private let style: Style.Type
        private let label: String
        private let placeholder: String
        private let suggestions: [String]
        private let rootStack = UIStackView()
        private let chipScrollView = UIScrollView()
        private let chipStack = UIStackView()
        private let textField = UITextField()
        private let suggestionStack = UIStackView()
        private var selectedTags: [String]

        var tags: [String] {
            selectedTags
        }

        init(
            label: String,
            placeholder: String,
            initialTags: [String],
            suggestions: [String],
            style: Style.Type
        ) {
            self.label = label
            self.placeholder = placeholder
            self.suggestions = suggestions
            self.style = style
            self.selectedTags = TagFieldView.normalizedTagNames(initialTags)
            super.init(frame: .zero)
            buildLayout()
            reloadChips()
            reloadSuggestions()
        }

        @available(*, unavailable)
        required init?(coder: NSCoder) {
            fatalError("init(coder:) has not been implemented")
        }

        private static func normalizedTagName(_ rawName: String) -> String? {
            let trimmed = rawName
                .trimmingCharacters(in: .whitespacesAndNewlines)
                .trimmingCharacters(in: CharacterSet(charactersIn: "#"))
                .trimmingCharacters(in: .whitespacesAndNewlines)

            guard !trimmed.isEmpty else {
                return nil
            }

            let isValid = trimmed.unicodeScalars.allSatisfy { scalar in
                scalar.properties.isAlphabetic ||
                    scalar.properties.numericType != nil ||
                    scalar.value == 95 ||
                    scalar.value == 45
            }

            return isValid ? trimmed : nil
        }

        private static func normalizedTagNames(_ rawNames: [String]) -> [String] {
            var names: [String] = []
            var seen = Set<String>()

            for rawName in rawNames {
                guard let name = normalizedTagName(rawName) else {
                    continue
                }

                let key = name.lowercased()
                if seen.insert(key).inserted {
                    names.append(name)
                }
            }

            return names
        }

        private func buildLayout() {
            translatesAutoresizingMaskIntoConstraints = false
            backgroundColor = style.paper
            layer.cornerRadius = 14
            layer.borderColor = style.ink.cgColor
            layer.borderWidth = 2

            rootStack.translatesAutoresizingMaskIntoConstraints = false
            rootStack.axis = .vertical
            rootStack.spacing = 8
            addSubview(rootStack)

            chipScrollView.translatesAutoresizingMaskIntoConstraints = false
            chipScrollView.showsHorizontalScrollIndicator = false
            chipScrollView.alwaysBounceHorizontal = false

            chipStack.translatesAutoresizingMaskIntoConstraints = false
            chipStack.axis = .horizontal
            chipStack.spacing = 8
            chipStack.alignment = .center
            chipScrollView.addSubview(chipStack)

            textField.placeholder = placeholder
            textField.font = .preferredFont(forTextStyle: .body)
            textField.adjustsFontForContentSizeCategory = true
            textField.textColor = style.ink
            textField.tintColor = style.accentSkyStrong
            textField.backgroundColor = .clear
            textField.accessibilityLabel = label
            textField.returnKeyType = .done
            textField.autocorrectionType = .default
            textField.delegate = self
            textField.addTarget(self, action: #selector(textDidChange), for: .editingChanged)
            textField.widthAnchor.constraint(greaterThanOrEqualToConstant: 120).isActive = true
            textField.heightAnchor.constraint(greaterThanOrEqualToConstant: 36).isActive = true

            suggestionStack.axis = .horizontal
            suggestionStack.spacing = 8
            suggestionStack.alignment = .leading

            rootStack.addArrangedSubview(chipScrollView)
            rootStack.addArrangedSubview(suggestionStack)

            NSLayoutConstraint.activate([
                rootStack.topAnchor.constraint(equalTo: topAnchor, constant: 10),
                rootStack.leadingAnchor.constraint(equalTo: leadingAnchor, constant: 12),
                rootStack.trailingAnchor.constraint(equalTo: trailingAnchor, constant: -12),
                rootStack.bottomAnchor.constraint(equalTo: bottomAnchor, constant: -10),

                chipStack.topAnchor.constraint(equalTo: chipScrollView.contentLayoutGuide.topAnchor),
                chipStack.leadingAnchor.constraint(equalTo: chipScrollView.contentLayoutGuide.leadingAnchor),
                chipStack.trailingAnchor.constraint(equalTo: chipScrollView.contentLayoutGuide.trailingAnchor),
                chipStack.bottomAnchor.constraint(equalTo: chipScrollView.contentLayoutGuide.bottomAnchor),
                chipStack.heightAnchor.constraint(equalTo: chipScrollView.frameLayoutGuide.heightAnchor),
                chipScrollView.heightAnchor.constraint(greaterThanOrEqualToConstant: 38)
            ])
        }

        private func reloadChips() {
            for view in chipStack.arrangedSubviews {
                chipStack.removeArrangedSubview(view)
                view.removeFromSuperview()
            }

            for tag in selectedTags {
                let button = UIButton(type: .system)
                var configuration = UIButton.Configuration.filled()
                configuration.title = "#\(tag)  ×"
                configuration.baseBackgroundColor = .white
                configuration.baseForegroundColor = style.ink
                configuration.cornerStyle = .capsule
                configuration.contentInsets = NSDirectionalEdgeInsets(
                    top: 6,
                    leading: 10,
                    bottom: 6,
                    trailing: 10
                )
                button.configuration = configuration
                button.accessibilityIdentifier = tag
                button.addTarget(self, action: #selector(removeTagButtonTapped(_:)), for: .touchUpInside)
                chipStack.addArrangedSubview(button)
            }

            chipStack.addArrangedSubview(textField)
        }

        private func reloadSuggestions() {
            for view in suggestionStack.arrangedSubviews {
                suggestionStack.removeArrangedSubview(view)
                view.removeFromSuperview()
            }

            let query = (textField.text ?? "").trimmingCharacters(in: .whitespacesAndNewlines).lowercased()
            guard !query.isEmpty else {
                suggestionStack.isHidden = true
                return
            }

            let selectedKeys = Set(selectedTags.map { $0.lowercased() })
            let matches = suggestions
                .filter { suggestion in
                    let key = suggestion.lowercased()
                    return !selectedKeys.contains(key) && key.contains(query)
                }
                .sorted { left, right in
                    let leftStarts = left.lowercased().hasPrefix(query)
                    let rightStarts = right.lowercased().hasPrefix(query)
                    if leftStarts != rightStarts {
                        return leftStarts
                    }
                    return left.localizedCaseInsensitiveCompare(right) == .orderedAscending
                }
                .prefix(3)

            suggestionStack.isHidden = matches.isEmpty

            for suggestion in matches {
                let button = UIButton(type: .system)
                var configuration = UIButton.Configuration.filled()
                configuration.title = "#\(suggestion)"
                configuration.baseBackgroundColor = style.canvas
                configuration.baseForegroundColor = style.inkMuted
                configuration.cornerStyle = .capsule
                configuration.contentInsets = NSDirectionalEdgeInsets(
                    top: 6,
                    leading: 10,
                    bottom: 6,
                    trailing: 10
                )
                button.configuration = configuration
                button.accessibilityIdentifier = suggestion
                button.addTarget(self, action: #selector(suggestionButtonTapped(_:)), for: .touchUpInside)
                suggestionStack.addArrangedSubview(button)
            }
        }

        private func commitTag(_ rawName: String) {
            guard let name = TagFieldView.normalizedTagName(rawName) else {
                return
            }

            let key = name.lowercased()
            guard !selectedTags.map({ $0.lowercased() }).contains(key) else {
                textField.text = ""
                reloadSuggestions()
                return
            }

            selectedTags.append(name)
            textField.text = ""
            reloadChips()
            reloadSuggestions()
            onChange?()
        }

        @objc private func textDidChange() {
            reloadSuggestions()
        }

        @objc private func removeTagButtonTapped(_ sender: UIButton) {
            guard let tag = sender.accessibilityIdentifier else {
                return
            }

            let key = tag.lowercased()
            selectedTags.removeAll { $0.lowercased() == key }
            reloadChips()
            reloadSuggestions()
            onChange?()
        }

        @objc private func suggestionButtonTapped(_ sender: UIButton) {
            guard let tag = sender.accessibilityIdentifier else {
                return
            }

            commitTag(tag)
        }

        func textFieldShouldReturn(_ textField: UITextField) -> Bool {
            commitTag(textField.text ?? "")
            return false
        }

        func textField(
            _ textField: UITextField,
            shouldChangeCharactersIn range: NSRange,
            replacementString string: String
        ) -> Bool {
            if string == " " || string == "," {
                commitTag(textField.text ?? "")
                return false
            }

            return true
        }
    }

    private func emitResult(
        status: String,
        value: String?,
        values: [String: TicklyNativeSheetValue]?,
        actionId: String?
    ) {
        guard let webView else {
            return
        }

        let result = TicklyNativeSheetResult(
            token: request.token,
            status: status,
            value: value,
            values: values,
            actionId: actionId
        )

        guard let data = try? JSONEncoder().encode(result),
              let json = String(data: data, encoding: .utf8)
        else {
            return
        }

        let script = """
        window.dispatchEvent(new CustomEvent("tickly:nativeSheetResult", { detail: \(json) }));
        """
        webView.evaluateJavaScript(script)
    }
}

@available(iOS 15.0, *)
private final class TicklyLeafSheetTransitioningDelegate: NSObject, UIViewControllerTransitioningDelegate {
    func presentationController(
        forPresented presented: UIViewController,
        presenting: UIViewController?,
        source: UIViewController
    ) -> UIPresentationController? {
        TicklyLeafSheetPresentationController(
            presentedViewController: presented,
            presenting: presenting
        )
    }

    func animationController(
        forPresented presented: UIViewController,
        presenting: UIViewController,
        source: UIViewController
    ) -> UIViewControllerAnimatedTransitioning? {
        TicklyLeafSheetAnimator(isPresenting: true)
    }

    func animationController(forDismissed dismissed: UIViewController) -> UIViewControllerAnimatedTransitioning? {
        TicklyLeafSheetAnimator(isPresenting: false)
    }
}

@available(iOS 15.0, *)
private final class TicklyLeafSheetPresentationController: UIPresentationController {
    private let dimmingView = UIView()
    private var keyboardOverlap: CGFloat = 0
    private var panGestureRecognizer: UIPanGestureRecognizer?

    override init(presentedViewController: UIViewController, presenting presentingViewController: UIViewController?) {
        super.init(
            presentedViewController: presentedViewController,
            presenting: presentingViewController
        )

        dimmingView.backgroundColor = UIColor.black.withAlphaComponent(0.42)
        dimmingView.alpha = 0
        let tapGesture = UITapGestureRecognizer(target: self, action: #selector(dimmingViewTapped))
        dimmingView.addGestureRecognizer(tapGesture)
    }

    deinit {
        NotificationCenter.default.removeObserver(self)
    }

    override var frameOfPresentedViewInContainerView: CGRect {
        guard let containerView else {
            return .zero
        }

        let bounds = containerView.bounds
        let safeInsets = containerView.safeAreaInsets
        let horizontalInset: CGFloat = traitCollection.horizontalSizeClass == .compact ? 12 : 24
        let width = min(bounds.width - horizontalInset * 2, 440)
        let bottomGap: CGFloat = keyboardOverlap > 0 ? 8 : 0
        let bottomY = bounds.maxY - keyboardOverlap - bottomGap
        let topLimit = bounds.minY + safeInsets.top + 18
        let maxHeight = max(180, bottomY - topLimit)
        let minimumHeight = min(260, maxHeight)
        let safeBottomHeight = keyboardOverlap > 0 ? 0 : safeInsets.bottom
        let preferredHeight = presentedViewController.preferredContentSize.height + safeBottomHeight
        let height = min(max(preferredHeight, minimumHeight), maxHeight)
        let x = bounds.midX - width / 2
        let y = bottomY - height

        return CGRect(x: x, y: y, width: width, height: height)
    }

    override func presentationTransitionWillBegin() {
        guard let containerView else {
            return
        }

        dimmingView.frame = containerView.bounds
        containerView.insertSubview(dimmingView, at: 0)
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

        presentedViewController.transitionCoordinator?.animate { [weak self] _ in
            self?.dimmingView.alpha = 1
        }
    }

    override func presentationTransitionDidEnd(_ completed: Bool) {
        if completed {
            installPanGestureIfNeeded()
        } else {
            dimmingView.removeFromSuperview()
            NotificationCenter.default.removeObserver(self)
        }
    }

    override func dismissalTransitionWillBegin() {
        presentedViewController.transitionCoordinator?.animate { [weak self] _ in
            self?.dimmingView.alpha = 0
        }
    }

    override func dismissalTransitionDidEnd(_ completed: Bool) {
        if completed {
            dimmingView.removeFromSuperview()
            NotificationCenter.default.removeObserver(self)
        }
    }

    override func containerViewWillLayoutSubviews() {
        super.containerViewWillLayoutSubviews()

        dimmingView.frame = containerView?.bounds ?? .zero
        presentedView?.frame = frameOfPresentedViewInContainerView
    }

    private func installPanGestureIfNeeded() {
        guard panGestureRecognizer == nil,
              let presentedView
        else {
            return
        }

        let panGestureRecognizer = UIPanGestureRecognizer(
            target: self,
            action: #selector(panGestureRecognized(_:))
        )
        panGestureRecognizer.cancelsTouchesInView = false
        presentedView.addGestureRecognizer(panGestureRecognizer)
        self.panGestureRecognizer = panGestureRecognizer
    }

    @objc private func dimmingViewTapped() {
        (presentedViewController as? TicklyNativeSheetViewController)?.requestCancelFromPresentation()
    }

    @objc private func panGestureRecognized(_ recognizer: UIPanGestureRecognizer) {
        guard let presentedView else {
            return
        }

        let translation = recognizer.translation(in: containerView)
        let velocity = recognizer.velocity(in: containerView)
        let translationY = max(0, translation.y)

        switch recognizer.state {
        case .changed:
            presentedView.transform = CGAffineTransform(translationX: 0, y: translationY)
            dimmingView.alpha = max(0.62, 1 - translationY / 420)
        case .ended, .cancelled:
            let shouldDismiss = translationY > 92 || velocity.y > 900
            if shouldDismiss {
                (presentedViewController as? TicklyNativeSheetViewController)?.requestCancelFromPresentation()
            } else {
                UIView.animate(
                    withDuration: 0.22,
                    delay: 0,
                    usingSpringWithDamping: 0.88,
                    initialSpringVelocity: 0,
                    options: [.beginFromCurrentState, .allowUserInteraction]
                ) {
                    presentedView.transform = .identity
                    self.dimmingView.alpha = 1
                }
            }
        default:
            break
        }
    }

    @objc private func keyboardWillChangeFrame(_ notification: Notification) {
        guard let containerView,
              let endFrame = notification.userInfo?[UIResponder.keyboardFrameEndUserInfoKey] as? CGRect
        else {
            return
        }

        let convertedFrame = containerView.convert(endFrame, from: nil)
        keyboardOverlap = max(0, containerView.bounds.maxY - convertedFrame.minY)
        animateFrameChange(with: notification)
    }

    @objc private func keyboardWillHide(_ notification: Notification) {
        keyboardOverlap = 0
        animateFrameChange(with: notification)
    }

    private func animateFrameChange(with notification: Notification) {
        guard let presentedView else {
            return
        }

        let duration = (notification.userInfo?[UIResponder.keyboardAnimationDurationUserInfoKey] as? NSNumber)?
            .doubleValue ?? 0.25
        let curveRaw = (notification.userInfo?[UIResponder.keyboardAnimationCurveUserInfoKey] as? NSNumber)?
            .uintValue ?? UIView.AnimationOptions.curveEaseInOut.rawValue
        let options = UIView.AnimationOptions(
            rawValue: curveRaw << 16 | UIView.AnimationOptions.beginFromCurrentState.rawValue
        )

        UIView.animate(
            withDuration: duration,
            delay: 0,
            options: options
        ) {
            presentedView.transform = .identity
            presentedView.frame = self.frameOfPresentedViewInContainerView
            self.dimmingView.frame = self.containerView?.bounds ?? .zero
        }
    }
}

@available(iOS 15.0, *)
private final class TicklyLeafSheetAnimator: NSObject, UIViewControllerAnimatedTransitioning {
    private let isPresenting: Bool

    init(isPresenting: Bool) {
        self.isPresenting = isPresenting
    }

    func transitionDuration(using transitionContext: UIViewControllerContextTransitioning?) -> TimeInterval {
        isPresenting ? 0.28 : 0.22
    }

    func animateTransition(using transitionContext: UIViewControllerContextTransitioning) {
        if isPresenting {
            animatePresentation(using: transitionContext)
        } else {
            animateDismissal(using: transitionContext)
        }
    }

    private func animatePresentation(using transitionContext: UIViewControllerContextTransitioning) {
        guard let toView = transitionContext.view(forKey: .to) else {
            transitionContext.completeTransition(false)
            return
        }

        let containerView = transitionContext.containerView
        let finalFrame = transitionContext.finalFrame(for: transitionContext.viewController(forKey: .to)!)
        toView.frame = finalFrame
        toView.transform = CGAffineTransform(translationX: 0, y: finalFrame.height + 24)
        toView.alpha = 0.96
        containerView.addSubview(toView)

        UIView.animate(
            withDuration: transitionDuration(using: transitionContext),
            delay: 0,
            usingSpringWithDamping: 0.9,
            initialSpringVelocity: 0.2,
            options: [.curveEaseOut, .allowUserInteraction]
        ) {
            toView.transform = .identity
            toView.alpha = 1
        } completion: { completed in
            transitionContext.completeTransition(completed && !transitionContext.transitionWasCancelled)
        }
    }

    private func animateDismissal(using transitionContext: UIViewControllerContextTransitioning) {
        guard let fromView = transitionContext.view(forKey: .from) else {
            transitionContext.completeTransition(false)
            return
        }

        let distance = fromView.bounds.height + 28
        UIView.animate(
            withDuration: transitionDuration(using: transitionContext),
            delay: 0,
            options: [.curveEaseIn, .beginFromCurrentState, .allowUserInteraction]
        ) {
            fromView.transform = CGAffineTransform(translationX: 0, y: distance)
            fromView.alpha = 0.98
        } completion: { completed in
            if transitionContext.transitionWasCancelled {
                fromView.transform = .identity
                fromView.alpha = 1
            }
            transitionContext.completeTransition(completed && !transitionContext.transitionWasCancelled)
        }
    }
}
