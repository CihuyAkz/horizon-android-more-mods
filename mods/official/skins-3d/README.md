# 3D Skins

- Native mod ID: `skins_3d`
- Supplied HNMOD version: `1.0.1`
- Original archive: `skins_3d-1.0.1.hnmod`
- Pinned Horizon revision: `294448014fba585f8d5d09dd8886f482493b6417`
- Pinned horizon-mixins revision: `ddd9700b21997c95d95ac6ee5ecafe3830cf622a`
- Required hook API: `1`

## Port status

Integrated into the Android client as a statically bundled native mod. Its Mods entry, reach setting, skin image lookup, frame-time cache population, and player render-layer geometry are wired into the Android renderer. It is not dynamically loaded from a runtime mods directory.

The port builds voxel geometry for the six outer skin layers (hat, jacket, sleeves and pants) when a skin texture is available. Skins outside the configured reach or without available pixels retain their normal flat outer layer.
