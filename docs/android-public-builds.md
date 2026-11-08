# Android public builds

Public APKs are built manually by the unified **Build Android APK** workflow. Open Actions → Build Android APK → Run workflow, choose `Public Builds`, and select one ABI. The workflow increments the patch component of `versionName`, increments the Android `versionCode`, updates the Android backend version, commits those changes to the selected branch, and then builds the audited, non-debuggable distribution APK.

## Signing — no repository Secrets required

The workflow uses the standard Android debug keystore generated automatically by `tools/android_build.py` at `.local/android/debug.keystore`. GitHub Actions caches this generated keystore between runs, so no custom keystore, password, or repository Actions Secret needs to be configured.

This signing identity is suitable for CI/testing distribution, but it is not a private production signing key for Google Play. If the GitHub Actions cache is deleted or unavailable, a newly generated certificate may require users to uninstall the previous APK before installing the new one.

## Public build procedure

1. Dispatch **Build Android APK** from a branch where the workflow's built-in `GITHUB_TOKEN` can push commits.
2. Select `Public Builds` and the target ABI.
3. Review the automated version-bump commit created by the workflow.
4. Download the APK artifact and check the version, ABI, source commit, signing certificate, and device compatibility before distribution.

Each run creates a new patch version and increments `versionCode`. Do not rerun an older Public Build workflow run: dispatch a new run so the version remains monotonic. An audited build is not a substitute for gameplay, GPU or device compatibility testing.
