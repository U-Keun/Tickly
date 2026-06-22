#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
APPLE_GEN_DIR="$ROOT_DIR/src-tauri/gen/apple"
TEMPLATE_DIR="$ROOT_DIR/src-tauri/ios-widget"
ICON_SOURCE="$ROOT_DIR/src-tauri/icons/icon.png"
APP_ICON_SET="$APPLE_GEN_DIR/Assets.xcassets/AppIcon.appiconset"

sync_ios_app_icons() {
  if [[ ! -f "$ICON_SOURCE" ]]; then
    echo "Missing app icon source: $ICON_SOURCE" >&2
    return
  fi

  if ! command -v sips >/dev/null 2>&1; then
    echo "Missing sips; skipping iOS app icon sync." >&2
    return
  fi

  mkdir -p "$APP_ICON_SET"

  local icon_specs=(
    "AppIcon-20x20@2x.png:40"
    "AppIcon-20x20@3x.png:60"
    "AppIcon-29x29@2x-1.png:58"
    "AppIcon-29x29@3x.png:87"
    "AppIcon-40x40@2x.png:80"
    "AppIcon-40x40@3x.png:120"
    "AppIcon-60x60@2x.png:120"
    "AppIcon-60x60@3x.png:180"
    "AppIcon-20x20@1x.png:20"
    "AppIcon-20x20@2x-1.png:40"
    "AppIcon-29x29@1x.png:29"
    "AppIcon-29x29@2x.png:58"
    "AppIcon-40x40@1x.png:40"
    "AppIcon-40x40@2x-1.png:80"
    "AppIcon-76x76@1x.png:76"
    "AppIcon-76x76@2x.png:152"
    "AppIcon-83.5x83.5@2x.png:167"
    "AppIcon-512@2x.png:1024"
  )

  local spec filename size
  for spec in "${icon_specs[@]}"; do
    filename="${spec%%:*}"
    size="${spec##*:}"
    sips -z "$size" "$size" "$ICON_SOURCE" --out "$APP_ICON_SET/$filename" >/dev/null
  done

  cat > "$APP_ICON_SET/Contents.json" <<'JSON'
{
  "images" : [
    {
      "size" : "20x20",
      "idiom" : "iphone",
      "filename" : "AppIcon-20x20@2x.png",
      "scale" : "2x"
    },
    {
      "size" : "20x20",
      "idiom" : "iphone",
      "filename" : "AppIcon-20x20@3x.png",
      "scale" : "3x"
    },
    {
      "size" : "29x29",
      "idiom" : "iphone",
      "filename" : "AppIcon-29x29@2x-1.png",
      "scale" : "2x"
    },
    {
      "size" : "29x29",
      "idiom" : "iphone",
      "filename" : "AppIcon-29x29@3x.png",
      "scale" : "3x"
    },
    {
      "size" : "40x40",
      "idiom" : "iphone",
      "filename" : "AppIcon-40x40@2x.png",
      "scale" : "2x"
    },
    {
      "size" : "40x40",
      "idiom" : "iphone",
      "filename" : "AppIcon-40x40@3x.png",
      "scale" : "3x"
    },
    {
      "size" : "60x60",
      "idiom" : "iphone",
      "filename" : "AppIcon-60x60@2x.png",
      "scale" : "2x"
    },
    {
      "size" : "60x60",
      "idiom" : "iphone",
      "filename" : "AppIcon-60x60@3x.png",
      "scale" : "3x"
    },
    {
      "size" : "20x20",
      "idiom" : "ipad",
      "filename" : "AppIcon-20x20@1x.png",
      "scale" : "1x"
    },
    {
      "size" : "20x20",
      "idiom" : "ipad",
      "filename" : "AppIcon-20x20@2x-1.png",
      "scale" : "2x"
    },
    {
      "size" : "29x29",
      "idiom" : "ipad",
      "filename" : "AppIcon-29x29@1x.png",
      "scale" : "1x"
    },
    {
      "size" : "29x29",
      "idiom" : "ipad",
      "filename" : "AppIcon-29x29@2x.png",
      "scale" : "2x"
    },
    {
      "size" : "40x40",
      "idiom" : "ipad",
      "filename" : "AppIcon-40x40@1x.png",
      "scale" : "1x"
    },
    {
      "size" : "40x40",
      "idiom" : "ipad",
      "filename" : "AppIcon-40x40@2x-1.png",
      "scale" : "2x"
    },
    {
      "size" : "76x76",
      "idiom" : "ipad",
      "filename" : "AppIcon-76x76@1x.png",
      "scale" : "1x"
    },
    {
      "size" : "76x76",
      "idiom" : "ipad",
      "filename" : "AppIcon-76x76@2x.png",
      "scale" : "2x"
    },
    {
      "size" : "83.5x83.5",
      "idiom" : "ipad",
      "filename" : "AppIcon-83.5x83.5@2x.png",
      "scale" : "2x"
    },
    {
      "size" : "1024x1024",
      "idiom" : "ios-marketing",
      "filename" : "AppIcon-512@2x.png",
      "scale" : "1x"
    }
  ],
  "info" : {
    "version" : 1,
    "author" : "xcode"
  }
}
JSON
}

if [[ ! -d "$APPLE_GEN_DIR" ]]; then
  echo "Missing iOS project directory: $APPLE_GEN_DIR" >&2
  echo "Run 'yarn tauri ios init' first." >&2
  exit 1
fi

mkdir -p "$APPLE_GEN_DIR/TicklyWidgetExtension"
mkdir -p "$APPLE_GEN_DIR/WidgetShared"
mkdir -p "$APPLE_GEN_DIR/Sources/tickly"

cp "$TEMPLATE_DIR/project.yml" "$APPLE_GEN_DIR/project.yml"
cp "$TEMPLATE_DIR/tickly_iOS.entitlements" "$APPLE_GEN_DIR/tickly_iOS/tickly_iOS.entitlements"
rm -f "$APPLE_GEN_DIR/Sources/tickly/TicklyNativeTextSheet.swift"
cp "$TEMPLATE_DIR/Sources/tickly/TicklyNativeSheet.swift" "$APPLE_GEN_DIR/Sources/tickly/TicklyNativeSheet.swift"
cp "$TEMPLATE_DIR/Sources/tickly/TicklyLiquidGlassDock.swift" "$APPLE_GEN_DIR/Sources/tickly/TicklyLiquidGlassDock.swift"
cp "$TEMPLATE_DIR/Sources/tickly/TicklyICloudSync.swift" "$APPLE_GEN_DIR/Sources/tickly/TicklyICloudSync.swift"
cp "$TEMPLATE_DIR/TicklyWidgetExtension/Info.plist" "$APPLE_GEN_DIR/TicklyWidgetExtension/Info.plist"
cp "$TEMPLATE_DIR/TicklyWidgetExtension/TicklyWidgetExtension.entitlements" "$APPLE_GEN_DIR/TicklyWidgetExtension/TicklyWidgetExtension.entitlements"
rm -f "$APPLE_GEN_DIR/TicklyWidgetExtension/CategoryWidgetConfigurationIntent.swift"
rm -f "$APPLE_GEN_DIR/TicklyWidgetExtension/RefreshWidgetIntent.swift"
rm -f "$APPLE_GEN_DIR/TicklyWidgetExtension/ToggleTodoIntent.swift"
rm -f "$APPLE_GEN_DIR/TicklyWidgetExtension/WidgetModels.swift"
rm -f "$APPLE_GEN_DIR/TicklyWidgetExtension/WidgetActionStore.swift"
rm -f "$APPLE_GEN_DIR/TicklyWidgetExtension/WidgetSnapshotLoader.swift"
cp "$TEMPLATE_DIR/TicklyWidgetExtension/TicklyWidgetBundle.swift" "$APPLE_GEN_DIR/TicklyWidgetExtension/TicklyWidgetBundle.swift"
cp "$TEMPLATE_DIR/TicklyWidgetExtension/TicklyWidget.swift" "$APPLE_GEN_DIR/TicklyWidgetExtension/TicklyWidget.swift"
cp "$TEMPLATE_DIR/TicklyWidgetExtension/TicklyLockScreenWidget.swift" "$APPLE_GEN_DIR/TicklyWidgetExtension/TicklyLockScreenWidget.swift"
find "$APPLE_GEN_DIR/WidgetShared" -mindepth 1 -delete
cp "$TEMPLATE_DIR/WidgetShared/CategoryWidgetConfigurationIntent.swift" "$APPLE_GEN_DIR/WidgetShared/CategoryWidgetConfigurationIntent.swift"
cp "$TEMPLATE_DIR/WidgetShared/RefreshWidgetIntent.swift" "$APPLE_GEN_DIR/WidgetShared/RefreshWidgetIntent.swift"
cp "$TEMPLATE_DIR/WidgetShared/ToggleTodoIntent.swift" "$APPLE_GEN_DIR/WidgetShared/ToggleTodoIntent.swift"
cp "$TEMPLATE_DIR/WidgetShared/WidgetModels.swift" "$APPLE_GEN_DIR/WidgetShared/WidgetModels.swift"
cp "$TEMPLATE_DIR/WidgetShared/WidgetActionStore.swift" "$APPLE_GEN_DIR/WidgetShared/WidgetActionStore.swift"
cp "$TEMPLATE_DIR/WidgetShared/WidgetSnapshotLoader.swift" "$APPLE_GEN_DIR/WidgetShared/WidgetSnapshotLoader.swift"

sync_ios_app_icons

(
  cd "$APPLE_GEN_DIR"
  xcodegen generate
)

echo "iOS widget files synced to src-tauri/gen/apple."
