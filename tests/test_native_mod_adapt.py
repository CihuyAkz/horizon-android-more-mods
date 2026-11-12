"""What native_mod_adapt.py changes in a mod's code (packaging/android/native_mod_adapt.py)."""
import importlib.util
import unittest
from pathlib import Path

ADAPT = Path(__file__).resolve().parents[1] / "packaging" / "android" / "native_mod_adapt.py"
spec = importlib.util.spec_from_file_location("native_mod_adapt", ADAPT)
adapt = importlib.util.module_from_spec(spec)
spec.loader.exec_module(adapt)

OLD_STATE = """Break(ItemRenderState {
    oversized_in_gui: resolved.oversized_in_gui,
    animated,
})"""
UPDATED_STATE = """Break(ItemRenderState {
    oversized_in_gui: resolved.oversized_in_gui,
    animated,
    cit_glint: None,
    hdr_tier: 0,
    hdr_light: 0,
})"""


class Code(unittest.TestCase):
    def test_a_mod_from_before_the_new_fields_gets_them(self):
        self.assertEqual(adapt.code(OLD_STATE).count("cit_glint: None"), 1)

    def test_a_mod_that_already_has_them_stays_as_it_is(self):
        # Eating Animation 0.2.0: a second copy is E0062 and no build.
        self.assertEqual(adapt.code(UPDATED_STATE), UPDATED_STATE)


class Calls(unittest.TestCase):
    def test_a_mod_calling_content_root_again_passes_what_it_now_takes(self):
        said = adapt.code("Break(content_root(root, enabled, &with))")
        self.assertEqual(said, "Break(content_root(root, enabled, &with, None, &mut |_| {}))")


class OldApi(unittest.TestCase):
    def test_the_old_mod_api_module_is_starlings(self):
        said = adapt.code("use horizon_client::mod_api::NativeScreenView;\nuse crate::mod_api;\n")
        self.assertEqual(
            said,
            "use horizon_client::starling_mod_api::NativeScreenView;\n"
            "use crate::starling_mod_api as mod_api;\n",
        )


GAME = """use super::Game;
impl Game<'_> {
    pub(in crate::game) fn native_tick(&mut self) {}
    /// A frame.
    pub(in crate::game) fn native_frame(&mut self, elapsed: Duration) {
        let _ = elapsed;
    }
    pub(in crate::game) fn native_gui_layer_post(&self, layer: &str, draw: &mut HudBatch) {
        self.fire(layer, draw);
    }
    pub(in crate::game) fn native_use_item_on(
        &mut self,
        state: BlockStateId,
        click: &Click,
    ) {
    }
    pub(in crate::game) fn native_use_item_on_old(
        &mut self,
        stack: &ItemStack,
        pos: [i32; 3],
    ) {
    }
}
impl fmt::Debug for Game<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}
pub fn draw_minimap(batch: &mut HudBatch) {}
"""


class Index(unittest.TestCase):
    def test_methods_are_read_with_their_parameters(self):
        found, _ = adapt.module_items(GAME)
        # A one-line body does not hide the method after it.
        self.assertIn(("Game", "native_frame"), found.methods)
        self.assertEqual(found.parameters[("Game", "native_tick")], ("&mut self", []))
        self.assertEqual(found.parameters[("Game", "native_frame")], ("&mut self", ["elapsed"]))
        self.assertEqual(
            found.parameters[("Game", "native_gui_layer_post")], ("&self", ["layer", "draw"])
        )
        self.assertEqual(found.parameters[("Game", "native_use_item_on")][1], ["state", "click"])
        self.assertNotIn(("", "fmt"), found.methods)
        self.assertNotIn(("Game", "fmt"), found.methods)
        self.assertIn("draw_minimap", found.functions)

    def test_mod_lines_keep_their_path(self):
        _, declared = adapt.module_items('#[path = "game/runtime/feature.rs"]\nmod feature;\nmod plain;\n')
        self.assertEqual(declared, [("feature", "game/runtime/feature.rs"), ("plain", None)])


def tree():
    found, _ = adapt.module_items(GAME)
    return {"horizon_client::game::native_hooks::base": found, "horizon_client::game::native_hooks": adapt.Module()}


class Relocate(unittest.TestCase):
    def test_a_moved_method_is_found_by_its_type_and_name(self):
        self.assertEqual(
            adapt.relocate("horizon_client::game::native_hooks::Game<'_>::native_frame", tree()),
            "horizon_client::game::native_hooks::base::Game<'_>::native_frame",
        )

    def test_what_is_nowhere_stays_as_it_was(self):
        target = "horizon_client::game::native_hooks::Game<'_>::native_gone"
        self.assertEqual(adapt.relocate(target, tree()), target)


INJECTION = """[[injection]]
id = "hud"
kind = "HEAD"
target = "horizon_client::game::native_hooks::base::Game<'_>::native_gui_layer_post"
helper = "hud.rs"
bindings = [
  { name = "batch", ty = "&mut HudBatch", value = "&mut *batch" },
]
"""


class Rebind(unittest.TestCase):
    def test_a_renamed_parameter_is_bound_by_its_new_name(self):
        said = adapt.rebind(INJECTION, tree(), {})
        self.assertIn('{ name = "batch", ty = "&mut HudBatch", value = "&mut *draw" }', said)

    def test_old_parameters_bind_to_the_old_function_starling_keeps(self):
        old = """[[injection]]
target = "horizon_client::game::native_hooks::base::Game<'_>::native_use_item_on"
bindings = [
  { name = "pos", ty = "[i32; 3]", value = "pos" },
]
"""
        said = adapt.rebind(old, tree(), {})
        self.assertIn('target = "horizon_client::game::native_hooks::base::Game<\'_>::native_use_item_on_old"', said)


class Includes(unittest.TestCase):
    def test_an_included_file_beside_it_is_written_in(self):
        import tempfile

        with tempfile.TemporaryDirectory() as root:
            folder = Path(root) / "crates/horizon-client/src/game/native_hooks"
            folder.mkdir(parents=True)
            (folder / "mod.rs").write_text('mod fire;\ninclude!(concat!(env!("OUT_DIR"), "/x.rs"));\ninclude!("shared.rs");\nfn after() {}\n')
            (folder / "shared.rs").write_text("fn shared() {}\n")
            adapt.inline_includes(Path(root))
            self.assertEqual(
                (folder / "mod.rs").read_text(),
                'mod fire;\ninclude!(concat!(env!("OUT_DIR"), "/x.rs"));\nfn shared() {}\nfn after() {}\n',
            )
            # Horizon's generator would read its items twice.
            self.assertFalse((folder / "shared.rs").exists())


if __name__ == "__main__":
    unittest.main()
