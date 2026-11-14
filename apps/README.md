# Applications

All applications share the root Cargo workspace and `Cargo.lock`:

| Package | Location | Target |
| --- | --- | --- |
| `web` | `apps/web` | `wasm32-unknown-unknown` |
| `desktop` | `apps/desktop/src-tauri` | Windows |
| `native` | `apps/native` | Windows |

Root Cargo commands still default to `math`. Use `cargo core-check` and `cargo core-test` for the shared crates; an unfiltered `--workspace` build cannot use a single target for all three apps. Each app's build script rejects the wrong target, and its runtime dependencies are target-specific.

## Zed

Run from the repository root, or use an absolute path to the script:

```sh
python3 tools/select_app.py core
python3 tools/select_app.py web
python3 tools/select_app.py desktop
python3 tools/select_app.py native
python3 tools/select_app.py desktop native
```

The selector updates `.zed/settings.json`, preserves unrelated settings, sets rust-analyzer's Cargo target, excludes inactive app source directories, and restricts both build-script runs and check-on-save to the selected packages. Web and Windows selections cannot be combined. Core is the checked-in default.

Cargo still resolves the single workspace lockfile and reads member manifests; the selection controls application source loading and compilation, not workspace membership. Zed restarts rust-analyzer when its initialization options change. Run Zed's Rust language server in WSL for every profile. Windows profiles use `cargo xwin check`; they do not require Windows Cargo.

## Web

Install the Wasm target and Trunk in the Rust environment used for the frontend:

```sh
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
cargo web-check --locked
cd apps/web
trunk serve
trunk build --release
```

`cargo web-check` only checks Rust compilation. Use Trunk to bundle the Wasm, JavaScript, CSS, and images. Trunk settings live in `apps/web/Trunk.toml`.

Trunk builds the browser distribution under `apps/web/dist/`. `index.html` copies processed images from root `img/`; it does not copy `img/raw/`. Enable the `tauri` feature when building the Tauri frontend. `--no-default-features` selects HTTP calculation instead of the local engine.

## Windows apps from WSL

All Rust and frontend commands run in WSL. No Windows Rust toolchain or launcher is required. The Windows MSVC cross-compiler is `cargo-xwin`, following [Tauri's cross-compilation guide](https://tauri.app/distribute/windows-installer/). Zigbuild currently does not support Windows targets.

Prerequisites in WSL:

```sh
rustup target add x86_64-pc-windows-msvc
cargo install --locked cargo-xwin
cargo install --locked tauri-cli
# Ubuntu system dependencies, when installing system-wide:
sudo apt install clang llvm lld nsis
```

On this workstation LLVM and NSIS are installed per user under `~/.local/share/tutorlolv2-cross/`, with launchers in `~/.cargo/bin/`; sudo is not needed to use them. cargo-xwin caches the Microsoft SDK/CRT in `~/.cache/cargo-xwin/`, separate from the game's patch archives.

```sh
cargo desktop-check --locked
cargo native-check --locked
cd apps/desktop
cargo tauri dev --target x86_64-pc-windows-msvc
cargo tauri build --target x86_64-pc-windows-msvc
```

The Tauri configuration selects a four-line Linux runner that invokes `cargo-xwin` and invokes Trunk directly in `apps/web`. `dev` serves the frontend on port 8000; `build` bundles its release frontend from `apps/web/dist/`. Use `--debug --no-bundle` for a linked debug executable without producing an installer. The runner sets `CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_RUNNER=/usr/bin/env` before invoking cargo-xwin, so Windows binaries launch through WSL interop rather than Wine. For direct `cargo xwin run` or `test` calls, supply that same environment variable.

Native raster images default to the repository's `img/raw/`. `TUTORLOLV2_RAW_IMG` can override that location. Existing application IDs, local-storage locations, environment variables, and API routes remain unchanged. Building the engine still requires the root game-data cache.

## Imported history

The source commit IDs below are preserved without rewriting their histories:

| Application | Original repository | Imported commit |
| --- | --- | --- |
| Web | [tutorlolv2_web](https://github.com/LuizGomes56/tutorlolv2_web) | `6fc01439a9d3ce48581a406e0941cb32b1b4dc4e` |
| Desktop | [tutorlolv2_desktop_app](https://github.com/LuizGomes56/tutorlolv2_desktop_app) | `b407f9644a3e6ec55f9a32288f7264da950cdab2` |
| Native | [tutorlolv2_native](https://github.com/LuizGomes56/tutorlolv2_native) | `1e6d44100a29cb2300988ef9cbe5161c4c2835a8` |

The migration also includes the Windows checkouts' uncommitted tracked changes at import time. The original working directories were not modified. Their existing histories retain the original paths and package names; use the source commits when inspecting pre-import history.

Integrate the import using a normal merge commit or fast-forward. Squashing or rebasing away the import merge would discard the original repositories’ ancestry from the resulting branch.
