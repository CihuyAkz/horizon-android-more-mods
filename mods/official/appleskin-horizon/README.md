# AppleSkin Horizon

- Native mod ID: `appleskin_horizon`
- Supplied HNMOD version: `1.0.0`
- Original archive: `appleskin_horizon-1.0.0.hnmod`
- Pinned Horizon revision: `ba6344a59cb9e755b730dcf02285fc0d33b97d3c`
- Pinned horizon-mixins revision: `3f99a5c60dfa311cc365524c44ae2fd799bfa1de`
- Required hook API: `1`

## Port status

Integrated into the Android client as a statically bundled native mod. Its Mods entry, settings toggle, and saturation/held-food HUD overlay are wired through `android_bundled.rs` and `native_hud`. It is not dynamically loaded from a runtime mods directory.

The Android HUD port draws the saturation overlay and held-food saturation preview. Exhaustion indicator parity is not included in this port.
