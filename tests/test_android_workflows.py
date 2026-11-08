"""Regression tests for Horizon Android GitHub Actions workflows."""
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]


class AndroidWorkflowTests(unittest.TestCase):
    def test_active_workflows_match_packaging_templates(self):
        pairs = {
            '.github/workflows/android-source.yml': 'packaging/android/source-checks.yml',
            '.github/workflows/android-build.yml': 'packaging/android/android-build.yml',
        }
        for active, template in pairs.items():
            with self.subTest(workflow=active):
                self.assertEqual((ROOT / active).read_bytes(), (ROOT / template).read_bytes())

    def test_single_build_workflow_has_requested_dispatch_options(self):
        path = ROOT / '.github/workflows/android-build.yml'
        text = path.read_text(encoding='utf-8')
        for value in ('name: Build Android APK', 'workflow_dispatch:',
                      'build_type:', 'Dev Builds', 'Public Builds', 'abi:',
                      'arm64-v8a', 'x86_64', 'armeabi-v7a', 'x86',
                      'Prevent Dev version downgrade', 'Bump public version',
                      'Commit public version bump', 'Prepare temporary development version',
                      'horizon-android-signing-${{ github.repository_id }}',
                      'python tools/android_prepare.py --java-home "$JAVA_HOME"',
                      '--dev-build "$DEV_BUILD_NUMBER"',
                      'horizon-android-public.apk', 'actions/upload-artifact@'):
            self.assertIn(value, text)
        self.assertNotIn('tags:', text)
        self.assertNotIn('push:', text)

    def test_old_split_android_build_workflows_are_removed(self):
        self.assertFalse((ROOT / '.github/workflows/android-dev.yml').exists())
        self.assertFalse((ROOT / '.github/workflows/android-public.yml').exists())
        self.assertFalse((ROOT / 'packaging/android/dev-builds.yml').exists())
        self.assertFalse((ROOT / 'packaging/android/public-builds.yml').exists())

    def test_build_workflow_requires_no_repository_secrets(self):
        text = (ROOT / '.github/workflows/android-build.yml').read_text(encoding='utf-8')
        self.assertNotIn('secrets.', text)
        self.assertNotIn('ANDROID_RELEASE_KEYSTORE', text)
        self.assertIn('horizon-android-signing-${{ github.repository_id }}', text)
        self.assertIn("git push origin \"HEAD:${GITHUB_REF_NAME}\"", text)
        self.assertIn('contents: write', text)

    def test_gradle_uses_generated_local_android_debug_keystore(self):
        gradle = (ROOT / 'packaging/android/build.gradle').read_text(encoding='utf-8')
        self.assertIn("file('../../.local/android/debug.keystore')", gradle)
        self.assertIn("storePassword 'android'", gradle)
        self.assertIn("keyAlias 'androiddebugkey'", gradle)
        self.assertNotIn('System.getenv(\'ANDROID_KEYSTORE_', gradle)

    def test_source_checks_validate_unified_workflow_template_sync(self):
        text = (ROOT / '.github/workflows/android-source.yml').read_text(encoding='utf-8')
        self.assertIn('cmp .github/workflows/android-source.yml packaging/android/source-checks.yml', text)
        self.assertIn('cmp .github/workflows/android-build.yml packaging/android/android-build.yml', text)
        self.assertNotIn('android-dev.yml', text)
        self.assertNotIn('android-public.yml', text)

    def test_ci_checks_hnmod_ports_and_compiles_official_wasm_mods(self):
        source = (ROOT / '.github/workflows/android-source.yml').read_text(encoding='utf-8')
        self.assertIn('python3 tools/check_hnmod_official_layout.py', source)
        self.assertIn('rustup target add wasm32-unknown-unknown', source)
        for package in ('far-view', 'fps-overlay', 'keystrokes', 'minimap', 'recipes', 'zoom'):
            self.assertIn(f'-p {package}', source)


if __name__ == '__main__':
    unittest.main()
