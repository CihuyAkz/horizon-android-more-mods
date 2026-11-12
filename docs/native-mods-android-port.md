# Additional Android native mod bundles

The following supplied HNMOD source bundles are included in the Starling Extras Android native-mod catalogue:

| Mod | Version | Horizon source pin |
| --- | --- | --- |
| Cinematic Interactions | 0.12.2 | `fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca` |
| First-person Model | 1.0.0 | `2ebf00a0281c592f79191e76fce0e7d522c6f425` |
| Eating Animation | 0.2.0 | `6d99e14ff0c70ae60bc56b67829b526409fa068e` |
| Veinminer | 0.3.2 | `d91cab68ded29bd2b004f7050c468fb98c41a00b` |
| World Map | 0.7.1 | `fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca` |

Each source archive is unpacked under `native-mods/<id>/`, with its `horizon-native.json`, mixin manifest, Rust helpers, and supplied assets retained. The Android asset bundler packages these folders as `.hnmod` archives. The five IDs are added to the Android build manifests and `native-mods/prebuilt.txt`. World Map replaces the former standalone `minimap` bundle; the shared terrain scanner and map-colour code in `crates/horizon-client-render/src/minimap.rs` remains because World Map uses it.

At native-build time, `packaging/android/native_build.sh` copies the selected bundles, pins the copied manifests to the Starling source revision, and runs `native_mod_adapt.py` before applying the mixins. This is the project's intended route for older Horizon HNMOD bundles; they are not copied into the game as runtime-loaded Rust source. The packages are made available to the Android native-mod build flow, not automatically compiled into the default APK library.

The supplied archives have different Horizon source pins. Those original pins are retained in each bundle's metadata for provenance; the native build flow retargets working copies to the Starling revision. The build requires the matching Rust, Cargo, and Android toolchains.

## License notes

- First-person Model includes its MIT license and attribution notice.
- Eating Animation includes its MIT license.
- Cinematic Interactions includes a notice that identifies the port as MIT and attributes the adapted hammer model under CC BY 4.0, but its notice refers to a `LICENSE` file that is absent from the supplied archive.
- Veinminer contains no license file or license declaration in the supplied archive. Keep it local and obtain clarification before redistributing it publicly.
- World Map 0.7.1 contains no license file or license declaration in the supplied HNMOD. Keep it local and obtain licensing clarification before redistributing it publicly. Its metadata's hooks requirement was adapted from 404 to this source tree's `NATIVE_HOOKS_API` 359; the manifest and compatibility source pin are kept aligned.

## Verification status

The archive structure, IDs, versions, source pins, manifest/helper references, Android bundle packaging, and generic microphone-permission detection are checked separately. Veinminer was also present in the successful Dev arm64 native build on 2026-11-12 (GitHub Actions run `43212568931`); this confirms it compiled in that build set. Installation and in-game behavior are not certified by source-only checks.
