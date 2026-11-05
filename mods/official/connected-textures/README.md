# Connected Textures (Android native official module)

- Mod ID: `connected_textures`
- Upstream HNMOD version: `0.1.2`
- Android implementation: statically linked into Horizon's renderer and native-mod hooks.
- In-game listing: Options → Mods (`connected_textures`).

This package is native Rust source, not a `horizon-mod-sdk` WASM component. Connected
Textures changes the renderer's section mesh output, which the current sandboxed WASM
render capability cannot modify. The Android build imports these files through
`crates/horizon-client-render/src/lib.rs` and
`crates/horizon-client/src/game/android_bundled.rs`.

`native_connected_textures.rs` contains the connected-texture rules and mesh adapter;
the other files provide Android frame, settings, list, and click integration.

The supplied HNMOD did not declare a license. Keep this local and do not redistribute
publicly until the mod author confirms the applicable license.
