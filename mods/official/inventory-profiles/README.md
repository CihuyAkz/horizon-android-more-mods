# Inventory Profiles

- Native mod ID: `inventory_profiles`
- Supplied HNMOD version: `1.0.0`
- Original archive: `inventory_profiles.hnmod`
- Pinned Horizon revision: `ba6344a59cb9e755b730dcf02285fc0d33b97d3c`
- Pinned horizon-mixins revision: `3f99a5c60dfa311cc365524c44ae2fd799bfa1de`
- Required hook API: `1`

## Port status

Integrated into the Android client as a statically bundled native mod. Its Mods entry, settings, key handling, per-frame behaviours, HUD notifications, and inventory sorting are wired through `android_bundled.rs` and `tweaks.rs`. It is not dynamically loaded from a runtime mods directory.

Auto-refill, tool-break protection, quick-stack, sort modes, and notifications are implemented by the imported runtime glue. Multiplayer/server acceptance still follows the game's normal inventory click rules.
