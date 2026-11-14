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

Cargo still resolves the single workspace lockfile and reads member manifests; the selection controls application source loading and compilation, not workspace membership. Zed restarts rust-analyzer when its initialization options change. Select the profile in the environment where Zed's language server runs: WSL for web/core, or Windows with the MSVC Rust toolchain for Windows development.

## Web

Install the Wasm target and Trunk in the Rust environment used for the frontend:

```sh
rustup target add wasm32-unknown-unknown
cargo web-check --locked
cargo web --locked
cd apps/web
trunk serve
```

Trunk builds the browser distribution under `apps/web/dist/`. `index.html` copies processed images from root `img/`; it does not copy `img/raw/`. Enable the `tauri` feature when building the Tauri frontend. `--no-default-features` selects HTTP calculation instead of the local engine.

## Windows apps

Use the Windows Rust toolchain and Visual Studio C++ build tools for MSVC builds:

```sh
cargo desktop-check --locked
cargo native-check --locked
cargo native --locked
```

For the Tauri app, run `apps\desktop\tauri.cmd dev` or `apps\desktop\tauri.cmd build` from a Windows shell. The wrapper maps the repository before invoking Cargo, so Tauri’s CMD hooks also work when the checkout is a WSL UNC path. From a normal Windows drive, `cargo tauri dev` or `cargo tauri build` in `apps/desktop/` also works. Its PowerShell hook locates `apps/web` relative to the repository. For a checkout accessed through a WSL UNC path, it invokes Trunk in the corresponding WSL distribution; for a Windows checkout, it uses Windows Trunk. The hook needs Trunk in that environment. The bundled frontend path is `apps/web/dist/`.

Native raster images default to the repository's `img/raw/`. `TUTORLOLV2_RAW_IMG` can override that location. Existing application IDs, local-storage locations, environment variables, and API routes remain unchanged.

Building the engine still requires the root game-data cache. These prerequisites are the same as for the shared crates.

## Imported history

The source commit IDs below are preserved without rewriting their histories:

| Application | Original repository | Imported commit |
| --- | --- | --- |
| Web | [tutorlolv2_web](https://github.com/LuizGomes56/tutorlolv2_web) | `6fc01439a9d3ce48581a406e0941cb32b1b4dc4e` |
| Desktop | [tutorlolv2_desktop_app](https://github.com/LuizGomes56/tutorlolv2_desktop_app) | `b407f9644a3e6ec55f9a32288f7264da950cdab2` |
| Native | [tutorlolv2_native](https://github.com/LuizGomes56/tutorlolv2_native) | `1e6d44100a29cb2300988ef9cbe5161c4c2835a8` |

The migration also includes the Windows checkouts' uncommitted tracked changes at import time. The original working directories were not modified. Their existing histories retain the original paths and package names; use the source commits when inspecting pre-import history.

Integrate the import using a normal merge commit or fast-forward. Squashing or rebasing away the import merge would discard the original repositories’ ancestry from the resulting branch.
