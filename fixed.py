
#!/usr/bin/env python3
# Apply the Android build fixes to the checked-out `android` branch.

from __future__ import annotations

import subprocess
import sys
from pathlib import Path


def run_git(*args: str, cwd: Path) -> str:
    result = subprocess.run(
        ["git", *args],
        cwd=cwd,
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    return result.stdout.strip()


def replace_once(content: str, before: str, after: str, label: str) -> str:
    count = content.count(before)
    if count != 1:
        raise RuntimeError(
            f"{label}: expected exactly one matching block, found {count}. "
            "No edited files have been written."
        )
    return content.replace(before, after, 1)


def main() -> int:
    root = Path(run_git("rev-parse", "--show-toplevel", cwd=Path.cwd()))
    branch = run_git("branch", "--show-current", cwd=root)
    if branch != "android":
        raise SystemExit(
            f"Refusing to patch branch {branch!r}; check out 'android' first."
        )

    updates: dict[str, str] = {}

    # Register Punchy's Android runtime and dedicated hand geometry renderer.
    path = "crates/horizon-client/src/game.rs"
    content = (root / path).read_text(encoding="utf-8")
    content = replace_once(
        content,
        '''#[path = "../../../mods/official/freecam/android.rs"]
pub(crate) mod freecam_hooks;
#[cfg(target_os = "android")]
mod android_input;''',
        '''#[path = "../../../mods/official/freecam/android.rs"]
pub(crate) mod freecam_hooks;
#[cfg(target_os = "android")]
#[path = "../../../mods/official/punchy/mixins/punchy/runtime.rs"]
mod punchy;
#[cfg(target_os = "android")]
#[path = "../../../mods/official/punchy/mixins/punchy/geometry.rs"]
mod punchy_geometry;
#[cfg(target_os = "android")]
mod android_input;''',
        "Punchy module declarations",
    )
    content = replace_once(
        content,
        '''    fn hand_geometry(&mut self, alpha: f64) -> Vec<Vertex> {
        let mut out = Vec::new();''',
        '''    fn hand_geometry(&mut self, alpha: f64) -> Vec<Vertex> {
        let mut out = Vec::new();
        #[cfg(target_os = "android")]
        if let Some(punchy_out) = android_bundled::punchy_hand_geometry(self, alpha) {
            return punchy_out;
        }''',
        "Punchy hand renderer hook",
    )
    updates[path] = content

    # Connect Punchy to the Android native-mod hooks already used by the bundle.
    path = "crates/horizon-client/src/game/android_bundled.rs"
    content = (root / path).read_text(encoding="utf-8")
    content = replace_once(
        content,
        '''        super::freecam_hooks::key(&mut *game, key, pressed);''',
        '''        super::freecam_hooks::key(&mut *game, key, pressed);
        super::punchy::key(&mut *game, key, pressed);''',
        "Punchy key hook",
    )
    content = replace_once(
        content,
        '''        super::freecam_hooks::list(&mut *mods);
        mods.push(NativeMod {''',
        '''        super::freecam_hooks::list(&mut *mods);
        mods.push(NativeMod {
            id: super::punchy::ID.into(),
            title: super::punchy::TITLE.into(),
            description: "Self-contained Rust animation preview. Animation data included.".into(),
            keys: vec![
                ["inspect".into(), "Inspect item".into(), "key.keyboard.f6".into()],
                ["reload".into(), "Reload animations".into(), "key.keyboard.f7".into()],
            ],
        });
        mods.push(NativeMod {''',
        "Punchy native mod listing",
    )
    content = replace_once(
        content,
        '''        super::freecam_hooks::settings(&mut *game, mod_id, &mut *buttons);''',
        '''        super::freecam_hooks::settings(&mut *game, mod_id, &mut *buttons);
        if mod_id == super::punchy::ID {
            super::punchy::settings(&*game, &mut *buttons);
        }''',
        "Punchy settings hook",
    )
    content = replace_once(
        content,
        '''        super::freecam_hooks::click(&mut *game, mod_id, button);''',
        '''        super::freecam_hooks::click(&mut *game, mod_id, button);
        if mod_id == super::punchy::ID {
            super::punchy::click(&*game, button);
        }''',
        "Punchy settings click hook",
    )
    content = replace_once(
        content,
        '''pub(super) fn native_frame(game: &mut Game<'_>, elapsed: Duration) {''',
        '''#[cfg(target_os = "android")]
pub(super) fn punchy_hand_geometry(game: &mut Game<'_>, alpha: f64) -> Option<Vec<Vertex>> {
    super::punchy::begin(game, alpha);
    if !super::punchy::ready(game) {
        return None;
    }
    let mut out = super::punchy_geometry::render(game, alpha);
    super::punchy::arms(game, alpha, &mut out);
    for vertex in &mut out {
        vertex.layer &= !horizon_client::gfx::OIT_ENTITY;
    }
    Some(out)
}

pub(super) fn native_frame(game: &mut Game<'_>, elapsed: Duration) {''',
        "Punchy hand geometry adapter",
    )
    updates[path] = content

    # Remove the obsolete Inventory Profiles adapter call; keep Horizon's sorter.
    path = "crates/horizon-client/src/game/tweaks.rs"
    content = (root / path).read_text(encoding="utf-8")
    content = replace_once(
        content,
        "use std::ops::ControlFlow;\n",
        "",
        "Inventory Profiles ControlFlow import",
    )
    content = replace_once(
        content,
        '''        #[cfg(target_os = "android")]
        if let ControlFlow::Break(()) =
            super::android_bundled::inventory_profiles_sort_slots(self, slots)
        {
            return;
        }
''',
        "",
        "Inventory Profiles adapter call",
    )
    updates[path] = content

    # Fix Freecam references to the external horizon-client crate.
    path = "mods/official/freecam/android.rs"
    content = (root / path).read_text(encoding="utf-8")
    content = replace_once(
        content,
        "use crate::Result;",
        "use horizon_client::Result;",
        "Freecam Result import",
    )
    updates[path] = content

    path = "mods/official/freecam/mixins/freecam/freeze.rs"
    content = (root / path).read_text(encoding="utf-8")
    content = replace_once(
        content,
        "crate::motion::freecam_gate::frozen()",
        "horizon_client::motion::freecam_gate::frozen()",
        "Freecam motion gate path",
    )
    updates[path] = content

    # Keep bundled-mod documentation consistent with the newly wired preview.
    path = "crates/horizon-client/src/game/android_bundled/README.md"
    content = (root / path).read_text(encoding="utf-8")
    content = replace_once(
        content,
        '''The supplied Punchy Rust Preview is not ported: its own README documents several
missing/experimental animation, renderer, physics, and compatibility systems; it
needs a dedicated renderer/hand-animation port and device testing.''',
        '''The supplied Punchy Rust Preview is wired as an Android runtime and hand-rendering
preview. Its own README documents remaining animation, renderer, physics, and
compatibility gaps, so device testing is still required.''',
        "Punchy documentation status",
    )
    updates[path] = content

    # Validate the Inventory Profiles package before changing anything.
    removed_dir = "mods/official/inventory-profiles"
    tracked = run_git("ls-files", "--", removed_dir, cwd=root).splitlines()

    if len(tracked) not in (0, 13):
        raise RuntimeError(
            f"Expected 0 or 13 tracked Inventory Profiles files, found {len(tracked)}. "
            "No edited files have been written."
        )

    # All text replacements are validated before any modifications are made.
    if tracked:
        subprocess.run(
            ["git", "rm", "-r", "--", removed_dir],
            cwd=root,
            check=True,
        )

    for relative_path, updated_content in updates.items():
        (root / relative_path).write_text(updated_content, encoding="utf-8")

    subprocess.run(["git", "diff", "--check"], cwd=root, check=True)

    print("Applied Android fixes.")
    print("\nUpdated files:")
    for relative_path in updates:
        print(f"  M {relative_path}")

    if tracked:
        print(f"\nDeleted Inventory Profiles package: {len(tracked)} tracked files")
    else:
        print("\nInventory Profiles package was already removed; skipped deletion.")

    print("\nReview the changes with: git diff")
    print("No commit was created.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (RuntimeError, subprocess.CalledProcessError) as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        raise SystemExit(1)
