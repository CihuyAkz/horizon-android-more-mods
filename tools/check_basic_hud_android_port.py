#!/usr/bin/env python3
"""Static checks for the Basic HUD Android native port (no Rust toolchain required)."""
from pathlib import Path
import re
import tomllib

ROOT = Path(__file__).resolve().parents[1]
MOD = ROOT / "mods/official/basic-hud/mixins/basic_hud"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def check_declared_paths(source: Path) -> None:
    text = source.read_text(encoding="utf-8")
    for rel in re.findall(r'^\s*#\[path\s*=\s*"([^"]+)"\]', text, re.MULTILINE):
        target = (source.parent / rel).resolve()
        require(target.is_file(), f"missing #[path] target in {source.relative_to(ROOT)}: {rel}")
    for rel in re.findall(r'include!\("([^"]+)"\)', text):
        target = (source.parent / rel).resolve()
        require(target.is_file(), f"missing include! target in {source.relative_to(ROOT)}: {rel}")


def main() -> None:
    manifest = tomllib.loads((MOD / "horizon-mixin.toml").read_text(encoding="utf-8"))
    for source in [
        ROOT / "crates/horizon-client/src/game.rs",
        ROOT / "crates/horizon-client/src/game/android_bundled.rs",
        ROOT / "crates/horizon-client/src/game/native_hooks.rs",
        ROOT / "crates/horizon-client/src/game/pickups.rs",
        ROOT / "crates/horizon-client/src/game/scoreboard.rs",
        ROOT / "crates/horizon-client/src/game/totem.rs",
        ROOT / "crates/horizon-client/src/game/names.rs",
        ROOT / "crates/horizon-client/src/game/remote_mob_data.rs",
        ROOT / "crates/horizon-client/src/game/remote.rs",
        ROOT / "crates/horizon-client/src/game/options.rs",
        ROOT / "crates/horizon-client/src/net.rs",
    ]:
        check_declared_paths(source)

    game = (ROOT / "crates/horizon-client/src/game.rs").read_text(encoding="utf-8")
    required_modules = {
        "basic_hud", "basic_hud_players", "basic_hud_i18n", "basic_hud_logic",
        "basic_hud_data", "basic_hud_render", "basic_hud_editor", "basic_hud_tab",
        "basic_hud_recent_pickups", "basic_hud_pickup_data",
    }
    for name in required_modules:
        require(re.search(rf'\bmod\s+{name}\s*;', game) is not None,
                f"missing Android module declaration: {name}")

    bundled = (ROOT / "crates/horizon-client/src/game/android_bundled.rs").read_text(encoding="utf-8")
    for call in [
        "basic_hud_key(", "basic_hud_listed(", "basic_hud_page(", "basic_hud_click(",
        "basic_hud_tick(", "basic_hud_frame(", "basic_hud_draw(",
    ]:
        require(call in bundled, f"missing Android bundle hook: {call}")

    hook_files = {
        "pickup_before": "native_hooks.rs",
        "pickup_local": "pickups.rs",
        "pickup_poll": "net.rs",
        "tab_list": "scoreboard.rs",
        "totem": "totem.rs",
        "name_tags": "names.rs",
        "health_packet": "remote_mob_data.rs",
        "entity_spawn_reset": "remote.rs",
        "entity_remove_reset": "remote.rs",
        "editor_draw": "options.rs",
        "editor_mouse": "options.rs",
        "editor_drag": "options.rs",
        "editor_keyboard": "options.rs",
        "editor_scroll": "options.rs",
        "editor_shortcut": "options.rs",
    }
    source_dir = ROOT / "crates/horizon-client/src/game"
    for injection, filename in hook_files.items():
        inj = next((item for item in manifest.get("injection", []) if item.get("id") == injection), None)
        require(inj is not None, f"manifest injection missing: {injection}")
        helper = inj.get("helper")
        require((MOD / helper).is_file(), f"upstream helper file missing: {helper}")
        text = (ROOT / "crates/horizon-client/src" / filename if filename == "net.rs" else source_dir / filename).read_text(encoding="utf-8")
        if injection == "pickup_poll":
            require("basic_hud_pickup_packets::observe(self, &events)" in text,
                    "pickup packet observer is not called by Session::poll")
            continue
        func = Path(helper).stem.replace("-", "_")
        require(func in text or f'include!("../../../../mods/official/basic-hud/mixins/basic_hud/{helper}")' in text,
                f"helper not wired from {filename}: {helper}")

    cooldowns = (ROOT / "crates/horizon-items/src/cooldowns.rs").read_text(encoding="utf-8")
    hud_data = (MOD / "data.rs").read_text(encoding="utf-8")
    require("pub fn entries(&self)" in cooldowns and "pub fn tick_count(&self)" in cooldowns,
            "read-only cooldown accessors are missing")
    require("game.cooldowns.entries()" in hud_data and "game.cooldowns.tick_count()" in hud_data,
            "Basic HUD cooldown adapter was not migrated to public accessors")
    require("__horizon_mixins_" not in hud_data,
            "private-field mixin bridge reference remains in Basic HUD data adapter")

    readme = (ROOT / "mods/official/basic-hud/README.md").read_text(encoding="utf-8")
    require("MIT" in readme and (ROOT / "mods/official/basic-hud/LICENSE").is_file(),
            "Basic HUD attribution/license notice missing")
    print("OK: Basic HUD modules, hook paths, Android call sites, cooldown accessors and license notice resolve")


if __name__ == "__main__":
    main()
