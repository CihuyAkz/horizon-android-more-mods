# Fixed Android mod sources

The parent `android_bundled.rs` wires a fixed, opt-in set of native modules into
existing game hook points. It is not a native-mod loader or compiler. The original
`zoom`, `minimap`, `recipes`, and `smartf3` modules are MIT-licensed; their
provenance and source revisions are recorded in `PROVENANCE.json`.

The runtime and adapter sources come from their corresponding upstream mods.
Helper files are renamed and expose `run` for static calls. The existing MIT
modules retain their original `horizon-mixin.toml` manifests for reference; these
are not used for Android building. This directory is not a directly installable
`.hnmod` bundle.

Android changes reset profile-scoped state on launcher transitions, retain the
Android debug text scale and preserve the code-of-conduct screen. Update the
portable runtime tests and Android checks when changing these adapters.


## User-supplied HNMOD ports (Android)

`mods/official/accurate-block-placement/` is statically wired to the current Android key, frame,
tick, and Mods settings hooks. Its block-use tick adapter lives in `game/remote.rs`
so it can call the existing private prediction methods without exposing them. The
supplied archive has no license file/declaration for the HNMOD's Rust port itself
(its runtime identifies an MIT-licensed upstream project); keep this port local
until the HNMOD author clarifies its license.

`mods/official/connected-textures/` is statically wired to the current section mesher and Mods
settings hooks. The renderer applies CTM to the completed solid/translucent quads
after the native mesher has finished. The supplied HNMOD archive contains no
license declaration for Connected Textures; its source is retained here only for
the requesting user's local integration. Do not redistribute this port unless
its author grants permission or supplies a compatible license.

The supplied Mod Menu mod is not separately bundled: this Android source already
provides an Options > Mods page with per-mod settings, so the HNMOD's alternate
menu would duplicate existing UI and its title-screen hooks are not a safe drop-in.
The supplied Punchy Rust Preview is not ported: its own README documents several
missing/experimental animation, renderer, physics, and compatibility systems; it
needs a dedicated renderer/hand-animation port and device testing.

The two user-supplied Android ports now live as separate native official source
packages under `mods/official/accurate-block-placement/` and
`mods/official/connected-textures/`. This adapter directory only owns the Android
hook bridge; `android_bundled.rs` imports the implementations directly from those
packages, and the normal native-mod menu registration keeps each as a separate row.
They are static native modules, not independently downloadable WASM `.hmod` files.

Additional static Android ports: Jade 26.3.6, ArmorHUD 1.0.0 and Resourcify 0.1.1.
