#!/bin/bash
# 生成 icns 的示例脚本
SRC="icon_1024.png"
ICONSET="AppIcon.iconset"
OUT_ICNS="MyApp.icns"

# 清理并创建 iconset 目录
rm -rf "$ICONSET"
mkdir -p "$ICONSET"

# 使用 sips 生成各尺寸（sips -z 高 宽）
sips -z 16 16    "$SRC" --out "$ICONSET/icon_16x16.png"
sips -z 32 32    "$SRC" --out "$ICONSET/icon_16x16@2x.png"
sips -z 32 32    "$SRC" --out "$ICONSET/icon_32x32.png"
sips -z 64 64    "$SRC" --out "$ICONSET/icon_32x32@2x.png"
sips -z 128 128  "$SRC" --out "$ICONSET/icon_128x128.png"
sips -z 256 256  "$SRC" --out "$ICONSET/icon_128x128@2x.png"
sips -z 256 256  "$SRC" --out "$ICONSET/icon_256x256.png"
sips -z 512 512  "$SRC" --out "$ICONSET/icon_256x256@2x.png"
sips -z 512 512  "$SRC" --out "$ICONSET/icon_512x512.png"
# 1024x1024 为 @2x 的 512
sips -z 1024 1024 "$SRC" --out "$ICONSET/icon_512x512@2x.png"

# 打包为 icns
iconutil -c icns "$ICONSET" -o "$OUT_ICNS"

# 可选：删除临时目录
rm -rf "$ICONSET"

echo "生成: $OUT_ICNS"