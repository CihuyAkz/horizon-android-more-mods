# Official mod sources

This tree contains two native source layouts:

- Existing folders with `Cargo.toml` and `horizon-mod.toml` are sandboxed
  `horizon-mod-sdk` components, built for Horizon's WASM mod runtime.
- `accurate-block-placement/` and `connected-textures/` are Android native
  source packages adapted from the supplied `.hnmod` bundles. They are statically
  linked into the Android game through `#[path]` imports and registered as
  separate `NativeMod` rows in Options > Mods. They are not standalone WASM
  `.hmod` files and are not discovered by scanning this directory at runtime.

The native packages have a `PORT.toml` and README rather than a `horizon-mod.toml`
manifest because their functionality depends on game-private client and renderer
hooks that the sandboxed WASM API does not expose.
