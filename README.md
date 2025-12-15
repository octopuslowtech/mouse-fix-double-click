# Mouse Fix Double Click

Desktop utility (Tauri 2 + Svelte 5) that suppresses ultra-fast double clicks OS-wide. The app hooks into global mouse events and drops the second click if it arrives too soon, helping mitigate worn mouse switches.

## Features

- Toggle the double-click guard on/off instantly.
- Toggle autostart with the OS.
- Live status and error messaging in a minimal dark UI.

## Platform support

- macOS: supported. Requires Accessibility permission on first enable.
- Windows / Linux: not supported yet. Current builds return `platform does not support global hooks` until native hooks are implemented.

## Requirements

- Bun ≥ 1.3 (https://bun.sh)
- Rust toolchain (macOS: Xcode Command Line Tools)
- Node toolchain compatible with Vite/Svelte (already covered by Bun)

## Setup

```bash
git clone https://github.com/YOUR_ORG/mouse-fix-double-click.git
cd mouse-fix-double-click
bun install
```

## Scripts

| Command | Purpose |
| --- | --- |
| `bun run tauri dev` | Run dev app with UI + Rust logs |
| `bun run tauri build` | Build native bundle |
| `bun run check` | Run `svelte-check` + `tsc` |
| `cd src-tauri && cargo check` | Check Rust commands/hooks |

## Usage

1. Run `bun run tauri dev`.
2. In the app, toggle **“Bật chặn double click”** to enable the guard. macOS will prompt for Accessibility; grant access.
3. Optionally toggle **“Khởi động cùng hệ thống”** to autostart.
4. If macOS blocks launch after first install, remove quarantine:  

```bash
sudo xattr -cr '/Applications/Mouse Fix Double Click.app/'
```


## Security & permissions

- Accessibility is required only to read/modify mouse events.
- Tauri commands stay within the default allowlist; no data collection.

## License

MIT – see [LICENSE](LICENSE).