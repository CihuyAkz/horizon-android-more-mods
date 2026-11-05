# Inventory Profiles

- Native mod ID: `inventory_profiles`
- Supplied HNMOD version: `1.0.0`
- Original archive: `inventory_profiles.hnmod`
- Pinned Horizon revision: `ba6344a59cb9e755b730dcf02285fc0d33b97d3c`
- Pinned horizon-mixins revision: `3f99a5c60dfa311cc365524c44ae2fd799bfa1de`
- Required hook API: `1`

## Port status

The complete supplied HNMOD source bundle is preserved in this folder. Its original `horizon-native.json`, `horizon-mixin.toml`, Rust helper files, crate sources, and bundled data were copied without source rewrites. This is a source import only: it is not yet statically wired into this Android build, and it is not a dynamically loadable Android mod. Copying it into `mods/official/` does not activate it.

A functional Android port still needs its hooks adapted to this source tree revision, registered with the current Android Mods UI, and validated by compiling and testing the Android build.
