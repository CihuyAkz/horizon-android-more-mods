# Official mod sources

This tree contains the Android-native ports and source-preserving imports used by Horizon.

## Android static native modules

- `accurate-block-placement/` and `connected-textures/` are Android-native source ports.
- `appleskin-horizon/` is wired into the Android HUD and Options > Mods.
- `freecam/` is partially wired into camera, input, player simulation and Options > Mods.
- `sodium-horizon/` is partially wired into terrain section visibility.
- Every bundled native module starts disabled; enable it explicitly in Options > Mods.

## Imported HNMOD sources

| Folder | Native mod ID | Version | Android status |
|---|---|---:|---|
| `freecam/` | `freecam` | 0.1.0 | partial Android runtime port; core camera/input hooks wired |
| `motion-blur/` | `motion_blur` | 0.1.0 | source-only, not wired |
| `punchy/` | `punchy` | 0.1.2 | source-only preview; renderer parity incomplete |
| `sodium-horizon/` | `sodium_horizon` | 0.3.1 | partial Android runtime port; section visibility only |

The imported HNMOD sources above are not Android runtime integrations: keeping an HNMOD folder under `mods/official/` does not by itself compile its modules or install its hooks. Their original manifests and Rust source are retained.

Voice Chat and Inventory Profiles were removed from this tree at the user's request. The 3D Skins mod remains removed.
