# Punchy

- Native mod ID: `punchy`
- Supplied HNMOD version: `0.1.2`
- Original archive: `punchy-rust-0.1.2.hnmod`
- Pinned Horizon revision: `81ae4a7d8f27d38e5cabbdcda0a06b2c4b000045`
- Pinned horizon-mixins revision: `ddd9700b21997c95d95ac6ee5ecafe3830cf622a`
- Required hook API: `1`

## Port status

The complete supplied HNMOD source bundle is preserved in this folder. Its original `horizon-native.json`, `horizon-mixin.toml`, Rust helper files, crate sources, and bundled data were copied without source rewrites. This is a source import only: it is not yet statically wired into this Android build, and it is not a dynamically loadable Android mod. Copying it into `mods/official/` does not activate it.

A functional Android port still needs its hooks adapted to this source tree revision, registered with the current Android Mods UI, and validated by compiling and testing the Android build.
