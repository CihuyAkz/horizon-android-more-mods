# Development APKs

Development APKs are built manually by the unified **Build Android APK** workflow. Open Actions → Build Android APK → Run workflow, choose `Dev Builds`, and select one ABI. The workflow uses the optimized Rust `release` profile and the public payload audit.

The package is `dev.horizon.android.dev`, separate from the normal release, and the visible version contains the GitHub Actions run number. Test data is separate; worlds and accounts are not automatically copied from the release app. Dev builds do not modify or commit the checked-in application version.

## Signing and installation

The build script creates the standard Android debug keystore at `.local/android/debug.keystore` when missing. GitHub Actions caches this generated file between runs, allowing subsequent CI builds to update an existing installation while the cache remains available. No repository Actions Secrets are required. If the cache is cleared and a new key is generated, Android may require uninstalling the previous app before installing the new one; back up test data first.

## Sharing with testers

Open Actions → Build Android APK → the successful run → Artifacts. The artifact contains the selected ABI's APK. GitHub requires sign-in to download workflow artifacts; the configured retention period is 14 or 30 days, depending on build type. Experimental builds have not automatically passed gameplay, GPU, or device compatibility tests.
