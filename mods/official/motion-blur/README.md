# Motion Blur

- Native mod ID: `motion_blur`
- Supplied HNMOD version: `0.1.0`
- Original archive: `motion_blur-0.1.0.hnmod`
- Pinned Horizon revision: `c9d474dd9116e84c243f8992d75c11314df56f06`
- Pinned horizon-mixins revision: `a2af5c0dd4117d51ecd70066ba7a00b4a2c10dab`
- Required hook API: `80`

## Port status

The complete supplied HNMOD source bundle is preserved in this folder. Its original `horizon-native.json`, `horizon-mixin.toml`, Rust helper files, crate sources, and bundled data were copied without source rewrites. This is a source import only: it is not yet statically wired into this Android build, and it is not a dynamically loadable Android mod. Copying it into `mods/official/` does not activate it.

A functional Android port still needs its hooks adapted to this source tree revision, registered with the current Android Mods UI, and validated by compiling and testing the Android build.
