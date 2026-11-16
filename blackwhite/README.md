# BlackWhite

The simplest Mac app: a window that is fully black or fully white.

- Click anywhere to switch between black and white.
- Resize it, stretch it, or fullscreen it (green button or ⌃⌘F). It always fills the window.
- Native Apple Silicon binary, written in Rust.
- No internet, no files, no settings, no tracking. About 60 lines of code in `src/main.rs`.

## Download

Grab `BlackWhite.dmg` from the latest GitHub Actions run (**Actions → Build BlackWhite DMG → Artifacts**),
open it, and drag BlackWhite into Applications.

The app is ad-hoc signed, not notarized, so on first launch macOS may block it.
Right-click the app → **Open** → **Open**, or run:

```bash
xattr -dr com.apple.quarantine /Applications/BlackWhite.app
```

## Build it yourself (on a Mac)

Needs [Rust](https://rustup.rs) and the Xcode command line tools.

```bash
./build-dmg.sh
```

Output: `dist/BlackWhite.dmg`.

## License

MIT
