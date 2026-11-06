//! Fixed Android mod bundle. No phone-side native-mod loader or compiler.

use super::*;

use super::native_hooks::*;

#[cfg(target_os = "android")]
#[path = "../../../../mods/official/accurate-block-placement/src/runtime.rs"]
pub(super) mod accurate_block_placement;

#[cfg(target_os = "android")]
#[path = "../../../../mods/official/accurate-block-placement/src/key.rs"]
mod accurate_block_placement_key;
#[cfg(target_os = "android")]
#[path = "../../../../mods/official/accurate-block-placement/src/list.rs"]
mod accurate_block_placement_list;
#[cfg(target_os = "android")]
#[path = "../../../../mods/official/accurate-block-placement/src/settings.rs"]
mod accurate_block_placement_settings;
#[cfg(target_os = "android")]
#[path = "../../../../mods/official/accurate-block-placement/src/click.rs"]
mod accurate_block_placement_click;
#[cfg(target_os = "android")]
#[path = "../../../../mods/official/accurate-block-placement/src/frame.rs"]
mod accurate_block_placement_frame;

#[cfg(target_os = "android")]
#[path = "../../../../mods/official/connected-textures/src/frame.rs"]
mod connected_textures_frame;
#[cfg(target_os = "android")]
#[path = "../../../../mods/official/connected-textures/src/list.rs"]
mod connected_textures_list;
#[cfg(target_os = "android")]
#[path = "../../../../mods/official/connected-textures/src/settings.rs"]
mod connected_textures_settings;
#[cfg(target_os = "android")]
#[path = "../../../../mods/official/connected-textures/src/click.rs"]
mod connected_textures_click;

#[cfg(target_os = "android")]
#[path = "android_bundled/zoom/runtime.rs"]
pub(super) mod official_zoom;

#[cfg(target_os = "android")]
#[path = "android_bundled/zoom/zoom_key.rs"]
mod zoom_key;

#[cfg(target_os = "android")]
#[path = "android_bundled/zoom/zoom_wheel.rs"]
mod zoom_wheel;

#[cfg(target_os = "android")]
#[path = "android_bundled/zoom/zoom_zoom.rs"]
mod zoom_zoom;

#[cfg(target_os = "android")]
#[path = "android_bundled/zoom/zoom_list.rs"]
mod zoom_list;

#[cfg(target_os = "android")]
#[path = "android_bundled/zoom/zoom_settings.rs"]
mod zoom_settings;

#[cfg(target_os = "android")]
#[path = "android_bundled/zoom/zoom_click.rs"]
mod zoom_click;

#[cfg(target_os = "android")]
#[path = "android_bundled/minimap/runtime.rs"]
pub(super) mod official_minimap;

#[cfg(target_os = "android")]
#[path = "android_bundled/minimap/minimap_list.rs"]
mod minimap_list;

#[cfg(target_os = "android")]
#[path = "android_bundled/minimap/minimap_settings.rs"]
mod minimap_settings;

#[cfg(target_os = "android")]
#[path = "android_bundled/minimap/minimap_click.rs"]
mod minimap_click;

#[cfg(target_os = "android")]
#[path = "android_bundled/minimap/minimap_minimap.rs"]
mod minimap_minimap;

#[cfg(target_os = "android")]
#[path = "android_bundled/recipes/runtime.rs"]
pub(super) mod official_recipes;

#[cfg(target_os = "android")]
#[path = "android_bundled/recipes/glue.rs"]
pub(super) mod official_recipes_glue;

#[cfg(target_os = "android")]
#[path = "android_bundled/recipes/recipes_list.rs"]
mod recipes_list;

#[cfg(target_os = "android")]
#[path = "android_bundled/recipes/recipes_settings.rs"]
mod recipes_settings;

#[cfg(target_os = "android")]
#[path = "android_bundled/recipes/recipes_click.rs"]
mod recipes_click;

#[cfg(target_os = "android")]
#[path = "android_bundled/recipes/recipes_frame.rs"]
mod recipes_frame;

#[cfg(target_os = "android")]
#[path = "android_bundled/recipes/recipes_event.rs"]
mod recipes_event;

#[cfg(target_os = "android")]
#[path = "android_bundled/smartf3/runtime.rs"]
pub(super) mod smartf3;

#[cfg(target_os = "android")]
#[path = "android_bundled/smartf3/native.rs"]
pub(super) mod smartf3_native;

#[cfg(target_os = "android")]
#[path = "android_bundled/smartf3/smartf3_command.rs"]
mod smartf3_command;

#[cfg(target_os = "android")]
#[path = "android_bundled/smartf3/smartf3_list.rs"]
mod smartf3_list;

#[cfg(target_os = "android")]
#[path = "android_bundled/smartf3/smartf3_settings.rs"]
mod smartf3_settings;

#[cfg(target_os = "android")]
#[path = "android_bundled/smartf3/smartf3_click.rs"]
mod smartf3_click;

pub(super) fn native_key(game: &mut Game<'_>, key: &str, pressed: bool) {
    let _ = (&game, &key, &pressed);
    #[cfg(target_os = "android")]
    {
        zoom_key::run(&mut *game, key, pressed);
        accurate_block_placement_key::run(&mut *game, key, pressed);
        super::freecam_hooks::key(&mut *game, key, pressed);
    }
}

pub(super) fn native_wheel(game: &mut Game<'_>, steps: f32, claimed: &mut bool) {
    let _ = (&game, &steps, &claimed);
    #[cfg(target_os = "android")]
    {
        zoom_wheel::run(&*game, steps, &mut *claimed);
    }
}

pub(super) fn native_zoom(game: &Game<'_>, zoom: &mut Option<NativeZoom>) {
    let _ = (&game, &zoom);
    #[cfg(target_os = "android")]
    {
        zoom_zoom::run(game, &mut *zoom);
    }
}

pub(super) fn native_mods(game: &mut Game<'_>, mods: &mut Vec<NativeMod>) {
    let _ = (&game, &mods);
    #[cfg(target_os = "android")]
    {
        zoom_list::run(&mut *mods);
        minimap_list::run(&mut *mods);
        recipes_list::run(&mut *mods);
        smartf3_list::run(&mut *mods);
        accurate_block_placement_list::run(&mut *mods);
        connected_textures_list::run(&mut *mods);
        super::mod_appleskin_glue::on_list(&mut *mods);
        super::freecam_hooks::list(&mut *mods);
        mods.push(NativeMod {
            id: "sodium_horizon".into(),
            title: "Sodium Horizon 0.3.1".into(),
            description: "Optional optimized section-visibility calculation".into(),
            keys: Vec::new(),
        });
    }
}

pub(super) fn native_settings(game: &mut Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    let _ = (&game, &mod_id, &buttons);
    #[cfg(target_os = "android")]
    {
        zoom_settings::run(&mut *game, mod_id, &mut *buttons);
        minimap_settings::run(&mut *game, mod_id, &mut *buttons);
        recipes_settings::run(&mut *game, mod_id, &mut *buttons);
        smartf3_settings::run(&mut *game, mod_id, &mut *buttons);
        accurate_block_placement_settings::run(&mut *game, mod_id, &mut *buttons);
        connected_textures_settings::run(mod_id, &mut *buttons);
        super::mod_appleskin_glue::on_settings(&mut *game, mod_id, &mut *buttons);
        super::freecam_hooks::settings(&mut *game, mod_id, &mut *buttons);
    }
}

pub(super) fn native_settings_click(game: &mut Game<'_>, mod_id: &str, button: usize) {
    let _ = (&game, &mod_id, &button);
    #[cfg(target_os = "android")]
    {
        zoom_click::run(&mut *game, mod_id, button);
        minimap_click::run(&mut *game, mod_id, button);
        recipes_click::run(&mut *game, mod_id, button);
        smartf3_click::run(&mut *game, mod_id, button);
        accurate_block_placement_click::run(&mut *game, mod_id, button);
        connected_textures_click::run(mod_id, button);
        super::mod_appleskin_glue::on_click(&mut *game, mod_id, button);
        super::freecam_hooks::click(&mut *game, mod_id, button);
    }
}

pub(super) fn native_minimap(game: &Game<'_>, minimap: &mut Option<NativeMinimap>) {
    let _ = (&game, &minimap);
    #[cfg(target_os = "android")]
    {
        minimap_minimap::run(game, &mut *minimap);
    }
}

pub(super) fn native_tick(game: &mut Game<'_>) {
    let _ = &game;
    #[cfg(target_os = "android")]
    {
        super::freecam_hooks::tick(&mut *game);
        horizon_client_render::native_sodium_horizon::set_enabled(
            game.native_on("sodium_horizon"),
        );
        if game.native_window() {
            game.android_accurate_block_placement_tick();
        }
    }
}

pub(super) fn native_frame(game: &mut Game<'_>, elapsed: Duration) {
    let _ = (&game, &elapsed);
    #[cfg(target_os = "android")]
    {
        super::freecam_hooks::frame(&*game, elapsed);
        recipes_frame::run(&mut *game);
        accurate_block_placement_frame::run(&mut *game);
        connected_textures_frame::run(&mut *game);
    }
}

pub(super) fn native_hud(game: &Game<'_>, batch: &mut horizon_client::hud::HudBatch) {
    #[cfg(target_os = "android")]
    super::freecam_hooks::hud(game, batch);
    let _ = (game, batch);
}

pub(super) fn native_panel_event(game: &mut Game<'_>, mod_id: &str, event: &NativePanelEvent) {
    let _ = (&game, &mod_id, &event);
    #[cfg(target_os = "android")]
    {
        recipes_event::run(&mut *game, mod_id, event);
    }
}

pub(super) fn native_command(game: &mut Game<'_>, command: &str, ran: &mut bool) {
    let _ = (&game, &command, &ran);
    #[cfg(target_os = "android")]
    {
        smartf3_command::run(&mut *game, command, &mut *ran);
    }
}

#[cfg(target_os = "android")]
pub(super) fn reset() {
    official_zoom::reset();
    official_minimap::reset();
    accurate_block_placement::forget();
    horizon_client_render::native_connected_textures::reset();
    official_recipes::set_viewer(None);
    smartf3_native::reset();
    horizon_client_render::native_sodium_horizon::set_enabled(false);
    let _ = super::freecam::disable();
    horizon_client::motion::freecam_gate::set_freeze(false);
}
