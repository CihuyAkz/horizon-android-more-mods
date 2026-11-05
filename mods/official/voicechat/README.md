# Voice Chat

- Native mod ID: `voicechat`
- Supplied HNMOD version: `1.0.0`
- Original archive: `voicechat-1.0.0.hnmod`
- Pinned Horizon revision: `c9d474dd9116e84c243f8992d75c11314df56f06`
- Pinned horizon-mixins revision: `ddd9700b21997c95d95ac6ee5ecafe3830cf622a`
- Required hook API: `1`

## Port status

The complete supplied HNMOD source bundle is preserved in this folder. Its original `horizon-native.json`, `horizon-mixin.toml`, Rust helper files, crate sources, and bundled data were copied without source rewrites. This is a source import only: it is not yet statically wired into this Android build, and it is not a dynamically loadable Android mod. Copying it into `mods/official/` does not activate it.

A functional Android port still needs its hooks adapted to this source tree revision, registered with the current Android Mods UI, and validated by compiling and testing the Android build.
