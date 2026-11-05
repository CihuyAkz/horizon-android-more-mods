# GitHub Actions setup — Horizon Android

This project has two active workflows under `.github/workflows/`:

- `android-source.yml`: source/unit checks on pushes and pull requests to `android`, validates the HNMOD Android ports, and compiles the existing official WASM mods.
- `android-build.yml`: the single manual APK workflow. Choose `Dev Builds` or `Public Builds`, then select one Android ABI. Dev builds do not modify the checked-in version; Public Builds increment the patch version and commit the version bump before building.

## Signing — no repository Secrets required

The build script automatically creates the standard Android debug keystore at `.local/android/debug.keystore` when it is missing. GitHub Actions caches that generated file between runs, so the same signing identity can be reused without adding values under repository Settings → Secrets and variables → Actions.

This is a development/debug signing identity, not a private Play Store production key. If the Actions cache is cleared, a new key may be generated; Android can then require uninstalling an older APK before installing the newly signed APK. Keep this limitation in mind when distributing Public Builds.

## Run the build

1. Open **Actions → Build Android APK → Run workflow**.
2. Select `Dev Builds` for a test build or `Public Builds` for a version-bumped distribution build.
3. Select `arm64-v8a`, `x86_64`, `armeabi-v7a`, or `x86`.
4. Download the APK from the successful run's Artifacts section.

Public Builds must be dispatched from a branch where the workflow's built-in `GITHUB_TOKEN` has permission to push the version bump. No custom token or repository Secret is needed. A successful CI build is not the same as gameplay/device testing.
