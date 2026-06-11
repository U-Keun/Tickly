#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
APPLE_GEN_DIR="$ROOT_DIR/src-tauri/gen/apple"
TEMPLATE_DIR="$ROOT_DIR/src-tauri/ios-widget"

if [[ ! -d "$APPLE_GEN_DIR" ]]; then
  echo "Missing iOS project directory: $APPLE_GEN_DIR" >&2
  echo "Run 'yarn tauri ios init' first." >&2
  exit 1
fi

mkdir -p "$APPLE_GEN_DIR/TicklyWidgetExtension"
mkdir -p "$APPLE_GEN_DIR/Sources/tickly"

cp "$TEMPLATE_DIR/project.yml" "$APPLE_GEN_DIR/project.yml"
cp "$TEMPLATE_DIR/tickly_iOS.entitlements" "$APPLE_GEN_DIR/tickly_iOS/tickly_iOS.entitlements"
rm -f "$APPLE_GEN_DIR/Sources/tickly/TicklyNativeTextSheet.swift"
cp "$TEMPLATE_DIR/Sources/tickly/TicklyNativeSheet.swift" "$APPLE_GEN_DIR/Sources/tickly/TicklyNativeSheet.swift"
cp "$TEMPLATE_DIR/Sources/tickly/TicklyLiquidGlassDock.swift" "$APPLE_GEN_DIR/Sources/tickly/TicklyLiquidGlassDock.swift"
cp "$TEMPLATE_DIR/TicklyWidgetExtension/Info.plist" "$APPLE_GEN_DIR/TicklyWidgetExtension/Info.plist"
cp "$TEMPLATE_DIR/TicklyWidgetExtension/TicklyWidgetExtension.entitlements" "$APPLE_GEN_DIR/TicklyWidgetExtension/TicklyWidgetExtension.entitlements"
cp "$TEMPLATE_DIR/TicklyWidgetExtension/TicklyWidgetBundle.swift" "$APPLE_GEN_DIR/TicklyWidgetExtension/TicklyWidgetBundle.swift"
cp "$TEMPLATE_DIR/TicklyWidgetExtension/TicklyWidget.swift" "$APPLE_GEN_DIR/TicklyWidgetExtension/TicklyWidget.swift"
cp "$TEMPLATE_DIR/TicklyWidgetExtension/TicklyLockScreenWidget.swift" "$APPLE_GEN_DIR/TicklyWidgetExtension/TicklyLockScreenWidget.swift"
cp "$TEMPLATE_DIR/TicklyWidgetExtension/CategoryWidgetConfigurationIntent.swift" "$APPLE_GEN_DIR/TicklyWidgetExtension/CategoryWidgetConfigurationIntent.swift"
cp "$TEMPLATE_DIR/TicklyWidgetExtension/RefreshWidgetIntent.swift" "$APPLE_GEN_DIR/TicklyWidgetExtension/RefreshWidgetIntent.swift"
cp "$TEMPLATE_DIR/TicklyWidgetExtension/ToggleTodoIntent.swift" "$APPLE_GEN_DIR/TicklyWidgetExtension/ToggleTodoIntent.swift"
cp "$TEMPLATE_DIR/TicklyWidgetExtension/WidgetModels.swift" "$APPLE_GEN_DIR/TicklyWidgetExtension/WidgetModels.swift"
cp "$TEMPLATE_DIR/TicklyWidgetExtension/WidgetActionStore.swift" "$APPLE_GEN_DIR/TicklyWidgetExtension/WidgetActionStore.swift"
cp "$TEMPLATE_DIR/TicklyWidgetExtension/WidgetSnapshotLoader.swift" "$APPLE_GEN_DIR/TicklyWidgetExtension/WidgetSnapshotLoader.swift"

(
  cd "$APPLE_GEN_DIR"
  xcodegen generate
)

echo "iOS widget files synced to src-tauri/gen/apple."
