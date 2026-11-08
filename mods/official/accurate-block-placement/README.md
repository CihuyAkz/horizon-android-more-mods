# Accurate Block Placement (Android native official module)

- Mod ID: `accurate_block_placement`
- Upstream HNMOD version: `1.0.0`
- Android implementation: statically linked into Horizon's native-mod hook points.
- In-game listing: Options → Mods (`accurate_block_placement`).

This package is native Rust source, not a `horizon-mod-sdk` WASM component. Android's
current implementation needs the game-private block-use and prediction path, which is
not exposed by the sandboxed WASM mod API. The Android build imports these files
through `crates/horizon-client/src/game/android_bundled.rs`.

The port keeps the mod's runtime logic separate from Horizon's adapter glue. `runtime.rs`
contains the portable placement state machine; the other files adapt it to Android's
input, frame, tick, settings, and Mod Menu hooks.

The supplied HNMOD did not declare a license for the port itself. Keep this local and
do not redistribute publicly until the mod author confirms the applicable license.
