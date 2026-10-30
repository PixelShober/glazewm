# GlazeWM for WinMarchy

This fork carries the window management that [WinMarchy](https://github.com/PixelShober/WinMarchy)
needs for an Omarchy-like desktop on Windows. It stays as close to upstream
[glzr-io/glazewm](https://github.com/glzr-io/glazewm) as possible, so upstream releases can be taken
over without much work.

## Branches

- `main` follows upstream `main` and carries nothing of its own.
- `fix/…` and `feat/…` are one change each, based on upstream `main`, so they can go upstream as pull
  requests.
- `winmarchy` is upstream plus every change below, and what releases are built from.

## Releases

A tag `v<upstream version>-winmarchy.<n>` (e.g. `v3.10.1-winmarchy.1`) builds
`glazewm-v…-x64.zip` with `glazewm.exe`, `glazewm-watcher.exe` and `cli\glazewm.exe`, the same
layout as the official standalone MSI. `glazewm query app-metadata` reports the full version, which
is how WinMarchy tells this build from the official one.

The binaries are not signed. UIAccess is therefore off: windows of programs that run as
administrator are not managed.

## Changes against upstream

| Change | Upstream |
|-|-|
| Redraw after `toggle-tiling-direction` / `set-tiling-direction`, so the new direction shows at once | pull request planned |
| On config reload, open workspaces move to the monitor in their `bind_to_monitor` | pull request planned |
| Build scripts read the numbers of pre-release versions (`3.10.1-winmarchy.1` → 3.10.1) | pull request planned |
| Redundant borrow that newer nightly clippy rejects (`user_config.rs`), which failed the lint check | pull request planned |
| The sample config starts no bar and ignores YASB's windows: WinMarchy starts YASB itself, and GlazeWM no longer brings Zebar along | fork only |
| This file and `.github/workflows/winmarchy.yaml` | fork only |
