//! Static Android wiring for the Gyro native mod.
include!("mixins/gyro/frame_helper.rs");
include!("mixins/gyro/key_helper.rs");
include!("mixins/gyro/list_helper.rs");
include!("mixins/gyro/turn_helper.rs");

#[path = "mixins/gyro/spin_helper.rs"]
mod spin;

pub(super) fn list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    on_mods(mods);
}

pub(super) fn key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    if game.native_on(horizon_client::gyro::ID) {
        on_key(game, key, pressed);
    }
}

pub(super) fn frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    if game.native_on(horizon_client::gyro::ID) {
        on_frame(game, elapsed);
    } else {
        use std::sync::atomic::Ordering;
        horizon_client::gyro::RUDDER_LEFT.store(false, Ordering::Relaxed);
        horizon_client::gyro::RUDDER_RIGHT.store(false, Ordering::Relaxed);
        horizon_client::gyro::set_roll(0.0);
    }
}

pub(super) fn mouse_look(game: &mut crate::game::Game<'_>) {
    on_mouse_look(game);
}

pub(super) fn spinning(
    game: &crate::game::Game<'_>,
    value: glam::Mat4,
) -> glam::Mat4 {
    if game.native_on(horizon_client::gyro::ID) {
        spin::on_return(game, value)
    } else {
        value
    }
}
