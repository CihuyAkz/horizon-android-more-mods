//! Android static wiring for the imported Freecam HNMOD helpers.
//!
//! The helper files are included verbatim from the HNMOD bundle. These thin
//! wrappers expose their injection points to the Android game glue.

use horizon_client::Result;
use horizon_client::hud::HudBatch;

include!("mixins/freecam/aim.rs");
include!("mixins/freecam/body.rs");
include!("mixins/freecam/camera.rs");
include!("mixins/freecam/click.rs");
include!("mixins/freecam/cycle.rs");
include!("mixins/freecam/frame.rs");
include!("mixins/freecam/freeze.rs");
include!("mixins/freecam/hud.rs");
include!("mixins/freecam/key.rs");
include!("mixins/freecam/list.rs");
include!("mixins/freecam/mapped.rs");
include!("mixins/freecam/mouse.rs");
include!("mixins/freecam/settings.rs");
include!("mixins/freecam/tick.rs");
include!("mixins/freecam/turn.rs");

pub(super) fn aim(
    game: &crate::game::Game<'_>,
) -> core::ops::ControlFlow<Option<([f64; 3], [f64; 3])>> {
    freecam_aim(game)
}

pub(super) fn body(game: &crate::game::Game<'_>) -> core::ops::ControlFlow<bool> {
    freecam_body(game)
}

pub(super) fn camera(
    game: &crate::game::Game<'_>,
) -> core::ops::ControlFlow<Option<(glam::DVec3, f32, f32)>> {
    freecam_camera(game)
}

pub(super) fn click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    freecam_click(game, mod_id, button);
}

pub(super) fn cycle(game: &crate::game::Game<'_>) -> core::ops::ControlFlow<()> {
    freecam_cycle(game)
}

pub(super) fn frame(game: &crate::game::Game<'_>, elapsed: std::time::Duration) {
    freecam_frame(game, elapsed);
}

pub(crate) fn freeze() -> core::ops::ControlFlow<Result<()>> {
    freecam_freeze()
}

pub(super) fn hud(game: &crate::game::Game<'_>, batch: &mut HudBatch) {
    freecam_hud(game, batch);
}

pub(super) fn key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    freecam_key(game, key, pressed);
}

pub(super) fn list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    freecam_listed(mods);
}

pub(super) fn mapped(
    game: &crate::game::Game<'_>,
    key: winit::keyboard::KeyCode,
    pressed: bool,
    repeat: bool,
) -> core::ops::ControlFlow<Result<()>> {
    freecam_mapped(game, key, pressed, repeat)
}

pub(super) fn mouse(
    game: &crate::game::Game<'_>,
    button: winit::event::MouseButton,
) -> core::ops::ControlFlow<Result<()>> {
    freecam_mouse(game, button)
}

pub(super) fn settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    freecam_page(game, mod_id, buttons);
}

pub(super) fn tick(game: &mut crate::game::Game<'_>) {
    freecam_tick(game);
}

pub(super) fn turn(
    game: &crate::game::Game<'_>,
    xo: f64,
    yo: f64,
) -> core::ops::ControlFlow<()> {
    freecam_turn(game, xo, yo)
}
