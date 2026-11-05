# HNMOD Android port status

This source tree does **not** add a phone-side `.hnmod` loader. The Android build
uses a fixed set of statically compiled Rust modules and the native hooks already
provided by Horizon. The source-only `.hnmod` archives therefore have to be
adapted to this tree's current APIs.

## Ported into the Android source

### Accurate Block Placement 1.0.0

- Moved the portable runtime and Android adapters to `mods/official/accurate-block-placement/src/`; the Android client imports them through `crates/horizon-client/src/game/android_bundled.rs`.
- Wired its keybind, Mods list row, settings page, setting persistence, frame sampling,
  and client tick processing into the current Android bundle.
- The tick adapter is located in `game/remote.rs` so it can reuse Horizon's existing
  private `remote_block_reacts`, `remote_use`, and local prediction path without
  widening those methods' visibility.
- Added a profile-state reset, so cached settings from another launcher profile are
  not carried forward.
- The supplied archive does not include a license file/declaration for the HNMOD's
  Rust port itself (its runtime identifies an MIT-licensed upstream dependency).
  Keep the port local until its author clarifies the port's license.

### Connected Textures 0.1.2

- Moved the renderer runtime to `mods/official/connected-textures/src/native_connected_textures.rs`; the renderer imports it from `crates/horizon-client-render/src/lib.rs`.
- Adapted its mesh pass to run on the current `Mesher::mesh_reusing` output, after
  Horizon finishes building the solid and translucent quad lists.
- Added its Mods row, settings page, `config/connected_textures.txt` persistence,
  profile reset, and resource-pack reload cache invalidation.
- The source archive supplied for this mod contains no license file or license
  declaration. It is therefore integrated only in this user's local working tree;
  do not redistribute this port unless the author grants permission or supplies a
  compatible license.

## Package layout and Mod Menu registration

The source of each port is now separated under:

- `mods/official/accurate-block-placement/`
- `mods/official/connected-textures/`

The Android adapters register `accurate_block_placement` and `connected_textures` as separate
`NativeMod` entries. They therefore have independent rows and settings pages in Options > Mods.
The implementation remains statically linked into the Android application; placing a folder in
`mods/official/` alone does not make Horizon dynamically discover it.

## Not separately ported

### Mod Menu 0.3.3

This Android source already has an Options > Mods page with mod rows and individual
settings pages. The supplied Mod Menu HNMOD replaces/extends menu/title-screen UI,
so it is not a safe drop-in and would duplicate the existing Mods entry point. The
existing Android Mods UI remains in place instead of adding a second menu layer.

### Punchy Rust Preview 0.1.2

The supplied preview's own README documents missing or experimental animation,
renderer, editor, physics, particles, and compatibility systems. It is tied to a
separate Horizon revision and needs a dedicated hand-animation/renderer port plus
runtime testing. It is not wired into the active Android build by this patch.

## Build and verification status

- Static source checks: the source-path, menu-registration, and metadata checks for the final layout pass.
- The final source package includes both relocated mod source trees and the updated Rust path declarations.
- Rust compilation, Android packaging, APK install, and in-game behavior: **not run**.
  The editing environment does not have `rustc`, `cargo`, the Android SDK/NDK, JDK,
  or Gradle configured. No compiled APK is included and compatibility on a device is
  not yet established.

## Apply the patch

From the root of the unmodified `horizon-android-more-mods-android` source directory:

```sh
patch -p1 < horizon-android-hnmod-port.patch
```

Then follow the project's `docs/android.md` build setup. For a public ARM64 build,
the documented flow is:

```sh
rustup target add aarch64-linux-android
python tools/android_prepare.py
python tools/android_build.py --abi arm64-v8a --public
```

The project documents Rust 1.95, Python 3.11+, Android SDK platform 35, Android
build-tools, NDK, a JDK, and Gradle 8.14. Build only after those prerequisites and
the project's private game-data preparation are configured.
