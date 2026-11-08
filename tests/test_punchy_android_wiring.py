from pathlib import Path
import re
import unittest

ROOT = Path(__file__).resolve().parents[1]


class HorizonAndroidIntegrationTests(unittest.TestCase):
    def test_punchy_renderer_is_connected_after_hide_hand_guard(self):
        source = (ROOT / "crates/horizon-client/src/game.rs").read_text()
        start = source.index("fn hand_geometry(")
        end = source.index("let a = alpha as f32;", start)
        section = source[start:end]
        guard = section.index("return out;")
        hook = section.index("android_bundled::punchy_hand_geometry(self, alpha)")
        self.assertLess(guard, hook)
        self.assertIn("return punchy_out;", section[hook:])

    def test_punchy_uses_main_mod_switch_and_swing_event(self):
        source = (ROOT / "mods/official/punchy/mixins/punchy/runtime.rs").read_text()
        enabled_start = source.index("fn enabled(")
        enabled_end = source.index("fn global_speed(", enabled_start)
        self.assertIn("game.native_on(ID)", source[enabled_start:enabled_end])
        self.assertNotIn('native_setting(ID, "enabled")', source)
        self.assertNotIn('native_set_setting(ID, "enabled"', source)
        self.assertIn('pub(super) const TITLE: &str = "Punchy";', source)
        tick_start = source.index("pub(super) fn tick(")
        tick_end = source.index("pub(super) fn key(", tick_start)
        tick = source[tick_start:tick_end]
        self.assertIn("load(&mut s);", tick)
        self.assertIn("let changed=game.swings_started!=s.previous_swings;", tick)
        self.assertIn("input.attack=(changed && !input.mining && !mining_attempt)", tick)

    def test_catalog_has_all_17_bundled_android_mods(self):
        catalog = (ROOT / "crates/horizon-client-render/src/mod_list.rs").read_text()
        bundled = (ROOT / "crates/horizon-client/src/game/android_bundled.rs").read_text()
        section = catalog.split("pub const ANDROID_BUNDLED:", 1)[1].split("/// The folder", 1)[0]
        ids = set(re.findall(r'^\s*\(?"([a-z0-9_]+)"', section, re.M))
        self.assertEqual(len(ids), 17)
        for token in (
            "zoom_list::run", "minimap_list::run", "recipes_list::run",
            "smartf3_list::run", "accurate_block_placement_list::run",
            "connected_textures_list::run", "mod_appleskin_glue::on_list",
            "armorhud_listed", "jade_listed", "resourcify_listed",
            "basic_hud_listed", "visuality_listed",
            "freecam_hooks::list", "freelook_hooks::list", "gyro_hooks::list",
            "waypoints_hooks::list", "punchy::ID",
        ):
            self.assertIn(token, bundled)

    def test_missing_mods_are_wired_to_android_input_and_frame_hooks(self):
        source = (ROOT / "crates/horizon-client/src/game/android_bundled.rs").read_text()
        for token in (
            "freelook_hooks::key", "gyro_hooks::key", "waypoints_hooks::key",
            "freelook_hooks::wheel", "freelook_hooks::frame",
            "gyro_hooks::frame", "waypoints_hooks::frame",
            "waypoints_hooks::settings_click", "waypoints_hooks::hud",
        ):
            self.assertIn(token, source)

