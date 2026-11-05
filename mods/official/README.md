# Official mod sources

This tree contains three native source layouts:

- Existing folders with `Cargo.toml` and `horizon-mod.toml` are sandboxed
  `horizon-mod-sdk` components, built for Horizon's WASM mod runtime.
- `accurate-block-placement/` and `connected-textures/` are Android-native
  source ports adapted to the current game and renderer APIs. They are statically
  linked into the Android game and registered separately in Options > Mods.
- The seven folders listed below are complete, source-preserving imports of the
  supplied `.hnmod` bundles. Each keeps its original `horizon-native.json`,
  `mixins/<id>/horizon-mixin.toml`, Rust helpers, crate sources, and bundled data.

## Imported HNMOD source bundles

| Folder | Native mod ID | Version |
|---|---|---:|
| `appleskin-horizon/` | `appleskin_horizon` | 1.0.0 |
| `freecam/` | `freecam` | 0.1.0 |
| `inventory-profiles/` | `inventory_profiles` | 1.0.0 |
| `motion-blur/` | `motion_blur` | 0.1.0 |
| `punchy/` | `punchy` | 0.1.2 |
| `sodium-horizon/` | `sodium_horizon` | 0.3.1 |
| `voicechat/` | `voicechat` | 1.0.0 |

These seven folders are **source-only imports**, not Android runtime integrations.
Their original manifests pin different Horizon revisions (and, for some bundles,
additional hook-point API requirements). This Android source statically links a
fixed set of modules; placing an HNMOD folder under `mods/official/` does not by
itself build it, add its hooks, or register it in Options > Mods. Each package's
`PORT.toml` and `README.md` records this status. The original bundle files are
preserved without rewriting their Rust code or manifests.

Some supplied bundles contain no license declaration for the mod source. The
files are retained in this user-requested local source tree; check with each
upstream author before redistributing those ports publicly. Punchy's supplied
bundle includes its own license files, which are retained in its package.
