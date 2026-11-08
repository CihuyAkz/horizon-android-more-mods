//! Android static wiring for the Waypoints native mod.
include!("mixins/waypoints/frame.rs");
include!("mixins/waypoints/hud.rs");
include!("mixins/waypoints/key.rs");
include!("mixins/waypoints/list.rs");
include!("mixins/waypoints/settings.rs");
include!("mixins/waypoints/click.rs");

pub(super) fn list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    waypoints_listed(mods);
}

pub(super) fn settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    waypoints_page(game, mod_id, buttons);
}

pub(super) fn settings_click(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    button: usize,
) {
    waypoints_click(game, mod_id, button);
}

pub(super) fn key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    waypoints_key(game, key, pressed);
}

pub(super) fn frame(game: &mut crate::game::Game<'_>) {
    waypoints_frame(game);
}

pub(super) fn hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
) {
    waypoints_hud(game, batch);
}
