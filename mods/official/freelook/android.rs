//! Static Android wiring for the Freelook native mod.
include!("mixins/freelook/camera.rs");
include!("mixins/freelook/key.rs");
include!("mixins/freelook/list.rs");
include!("mixins/freelook/mouse.rs");
include!("mixins/freelook/wheel.rs");

pub(super) fn key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    freelook_key(game, key, pressed);
}

pub(super) fn wheel(game: &crate::game::Game<'_>, steps: f32, claimed: &mut bool) {
    freelook_wheel(game, steps, claimed);
}

pub(super) fn list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    freelook_listed(mods);
}

pub(super) fn mouse(game: &mut crate::game::Game<'_>) -> std::ops::ControlFlow<()> {
    freelook_mouse(game)
}

pub(super) fn camera(
    game: &mut crate::game::Game<'_>,
    partial: f32,
) -> std::ops::ControlFlow<Option<(glam::DVec3, f32, f32)>> {
    freelook_camera(game, partial)
}

pub(super) fn frame(game: &mut crate::game::Game<'_>) {
    use crate::game::{freelook as m, perspective::CameraType};
    if game.native_on(m::ID) || !m::held() {
        return;
    }
    let _ = m::key(false, false);
    game.camera_type = match m::before() {
        1 => CameraType::ThirdPersonBack,
        2 => CameraType::ThirdPersonFront,
        _ => CameraType::FirstPerson,
    };
}
