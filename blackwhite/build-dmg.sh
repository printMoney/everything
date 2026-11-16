#!/bin/sh
# Builds BlackWhite.app and BlackWhite.dmg for Apple Silicon. Run on a Mac.
set -eu
cd "$(dirname "$0")"

rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin

APP=dist/BlackWhite.app
rm -rf dist
mkdir -p "$APP/Contents/MacOS"
cp target/aarch64-apple-darwin/release/blackwhite "$APP/Contents/MacOS/"
cp macos/Info.plist "$APP/Contents/"

# Ad-hoc signature (required to run on Apple Silicon).
codesign --force --deep --sign - "$APP"

mkdir -p dist/dmg
cp -R "$APP" dist/dmg/
ln -s /Applications dist/dmg/Applications
hdiutil create -volname BlackWhite -srcfolder dist/dmg -ov -format UDZO dist/BlackWhite.dmg
rm -rf dist/dmg

echo "Done: dist/BlackWhite.dmg"
