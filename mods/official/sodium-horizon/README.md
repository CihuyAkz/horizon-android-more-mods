# Sodium Horizon

- Native mod ID: `sodium_horizon`
- Supplied HNMOD version: `0.3.1`
- Original archive: `sodium_horizon-0.3.1.hnmod`
- Pinned Horizon revision: `c9d474dd9116e84c243f8992d75c11314df56f06`
- Pinned horizon-mixins revision: `4342a0bfbe71d4d14185528d64e496d6022f16a1`
- Required hook API: `1`

## Port status

The complete supplied HNMOD source bundle is preserved in this folder. Its original `horizon-native.json`, `horizon-mixin.toml`, Rust helper files, crate sources, and bundled data were copied without source rewrites. This is a source import only: it is not yet statically wired into this Android build, and it is not a dynamically loadable Android mod. Copying it into `mods/official/` does not activate it.

A functional Android port still needs its hooks adapted to this source tree revision, registered with the current Android Mods UI, and validated by compiling and testing the Android build.
