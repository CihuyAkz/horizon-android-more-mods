// SPDX-License-Identifier: MIT
// Original Sodium-style settings screen using Horizon's native renderer controls.
use super::{sodium_horizon_model as model, Game, Screen};
use horizon_client::{graphics as gfx, hud::HudBatch, options_ui::Page, settings::Settings};
use model::{hit, Editor, Layout, SPECS};
use std::cell::RefCell;
use winit::keyboard::KeyCode;

thread_local! { pub(super) static EDITOR: RefCell<Option<Editor>> = const { RefCell::new(None) }; }
const UPLOADS: [u32; 7] = [0, 1024, 2048, 4096, 8192, 16384, 32768];
const TEXTURES: [u32; 6] = [0, 128, 256, 512, 1024, 2048];

fn index(value: u32, choices: &[u32]) -> i32 {
    choices
        .iter()
        .position(|&v| v == value)
        .map_or(-(value as i32), |v| v as i32)
}

fn read_base(s: &Settings) -> Vec<i32> {
    vec![
        s.render_distance,
        s.simulation_distance,
        s.max_fps as i32,
        s.vsync as i32,
        s.fullscreen as i32,
        s.horizon.resolution_scale as i32,
        match s.clouds.as_str() {
            "false" => 0,
            "fast" => 1,
            _ => 2,
        },
        s.particles as i32,
        (s.entity_distance * 100.0).round() as i32,
        s.mipmap_levels,
        s.horizon.animated_textures as i32,
        s.cutout_leaves as i32,
        s.horizon.cave_culling as i32,
        s.horizon.entity_culling as i32,
        gfx::LeafCullingMode::ALL
            .iter()
            .position(|&v| v == s.horizon.leaf_culling)
            .unwrap_or(0) as i32,
        s.horizon.chunk_updates as i32,
        index(s.horizon.upload_budget_kib, &UPLOADS),
        s.horizon.workers as i32,
        s.horizon.translucent_batching as i32,
        gfx::PresentMode::ALL
            .iter()
            .position(|&v| v == s.horizon.present_mode)
            .unwrap_or(0) as i32,
        i32::from(s.horizon.pacing == gfx::Pacing::Precise),
        index(s.horizon.texture_budget_mib, &TEXTURES),
        match s.horizon.overlay {
            gfx::Overlay::Off => 0,
            gfx::Overlay::Fps => 1,
            gfx::Overlay::Graph => 2,
        },
        gfx::GraphicsApi::ALL
            .iter()
            .position(|&v| v == s.horizon.api)
            .unwrap_or(0) as i32,
    ]
}

// Write only the changed, explicitly enumerated non-lighting fields.
// Never replace the complete Settings or Graphics object with a preset.
fn write(s: &mut Settings, id: usize, v: i32) {
    match id {
        0 => s.render_distance = v,
        1 => s.simulation_distance = v,
        2 => s.max_fps = v as u32,
        3 => s.vsync = v != 0,
        4 => s.fullscreen = v != 0,
        5 => s.horizon.resolution_scale = v as u32,
        6 => s.clouds = ["false", "fast", "true"][v as usize].into(),
        7 => s.particles = v as u8,
        8 => s.entity_distance = v as f32 / 100.0,
        9 => s.mipmap_levels = v,
        10 => s.horizon.animated_textures = v != 0,
        11 => s.cutout_leaves = v != 0,
        12 => s.horizon.cave_culling = v != 0,
        13 => s.horizon.entity_culling = v != 0,
        14 => s.horizon.leaf_culling = gfx::LeafCullingMode::ALL[v as usize],
        15 => s.horizon.chunk_updates = v as u32,
        16 => s.horizon.upload_budget_kib = UPLOADS[v as usize],
        17 => s.horizon.workers = v as u32,
        18 => s.horizon.translucent_batching = v != 0,
        19 => s.horizon.present_mode = gfx::PresentMode::ALL[v as usize],
        20 => {
            s.horizon.pacing = if v == 0 {
                gfx::Pacing::Efficient
            } else {
                gfx::Pacing::Precise
            }
        }
        21 => s.horizon.texture_budget_mib = TEXTURES[v as usize],
        22 => {
            s.horizon.overlay =
                [gfx::Overlay::Off, gfx::Overlay::Fps, gfx::Overlay::Graph][v as usize]
        }
        23 => s.horizon.api = gfx::GraphicsApi::ALL[v as usize],
        170 => s.gui_scale = v as u32,
        _ => {}
    }
}

pub(super) fn active(game: &Game<'_>) -> bool {
    game.screen == Screen::Options
        && game.ui.options.page() == Page::Video
        && game.native_on("sodium_horizon")
}

fn edit<R>(game: &Game<'_>, f: impl FnOnce(&mut Editor) -> R) -> R {
    EDITOR.with_borrow_mut(|state| f(state.get_or_insert_with(|| Editor::new(read(game)))))
}

pub(super) fn frame(game: &Game<'_>) {
    sync(game);
    if !active(game) {
        EDITOR.with_borrow_mut(|state| *state = None);
        super::sodium_horizon_exact::clear();
    }
}

// Translation keys may be supplied by a resource pack; the original English
// catalogue stays here as a fallback rather than being baked into draw calls.

pub(super) fn draw(game: &Game<'_>, batch: &mut HudBatch) -> bool {
    super::sodium_horizon_exact::draw(game, batch)
}

pub(super) fn apply(game: &mut Game<'_>, ui: &mut Editor) {
    let changes = ui.changed();
    if changes.is_empty() {
        return;
    }
    ui.notice = changes.iter().any(|i| matches!(i, 17 | 18 | 21 | 23));
    for id in changes {
        match id {
            0..=23 | 170 => write(&mut game.settings, id, ui.draft[id]),
            32 => game.settings.horizon.sky = ui.draft[id] != 0,
            33 => game.settings.horizon.weather = ui.draft[id] != 0,
            34 => game.settings.horizon.fog = ui.draft[id] != 0,
            _ => {}
        }
    }
    let stored: std::collections::BTreeMap<&str, i32> = SPECS
        .iter()
        .enumerate()
        .skip(24)
        .map(|(i, s)| (s.key, ui.draft[i]))
        .collect();
    if let Ok(json) = serde_json::to_string(&stored) {
        game.native_set_setting("sodium_horizon", "extra-options-v1", &json);
    }
    EXTRA.with_borrow_mut(|cache| {
        *cache = Some(stored.into_iter().map(|(k, v)| (k.to_owned(), v)).collect())
    });
    sync_values(&ui.draft, game.native_on("sodium_horizon"));
    game.settings.horizon.preset = gfx::Preset::Custom;
    // The stock path applies renderer state, rebuilds meshes when necessary,
    // updates view distance and saves both vanilla and Horizon settings.
    game.settings.save(&Game::options_path());
    game.apply_settings();
    ui.commit();
}

pub(super) fn mouse(game: &mut Game<'_>, pressed: bool, right: bool, drag: bool) -> bool {
    super::sodium_horizon_exact::mouse(game, pressed, right, drag)
}

pub(super) fn scroll(game: &Game<'_>, amount: f32) -> bool {
    super::sodium_horizon_exact::scroll(game, amount)
}

pub(super) fn key(game: &mut Game<'_>, key: KeyCode, typed: Option<&str>) -> bool {
    super::sodium_horizon_exact::key(game, key, typed)
}

pub(super) fn type_text(game: &Game<'_>, typed: &str) -> bool {
    super::sodium_horizon_exact::type_text(game, typed)
}

thread_local! {
    static EXTRA: RefCell<Option<std::collections::BTreeMap<String,i32>>> = const {RefCell::new(None)};
    static SYNCED: RefCell<Option<bool>> = const {RefCell::new(None)};
}
fn defaults() -> Vec<i32> {
    let settings = Settings::default();
    let mut values = read_base(&settings);
    for spec in SPECS.iter().skip(24) {
        values.push(i32::from(spec.key != "coordinates"));
    }
    values[32] = settings.horizon.sky as i32;
    values[33] = settings.horizon.weather as i32;
    values[34] = settings.horizon.fog as i32;
    values[170] = settings.gui_scale as i32;
    values
}
pub(super) fn read(game: &Game<'_>) -> Vec<i32> {
    let mut values = read_base(&game.settings);
    EXTRA.with_borrow_mut(|cache| {
        let stored = cache.get_or_insert_with(|| {
            game.native_setting("sodium_horizon", "extra-options-v1")
                .and_then(|v| serde_json::from_str(&v).ok())
                .unwrap_or_default()
        });
        for spec in SPECS.iter().skip(24) {
            values.push(
                stored
                    .get(spec.key)
                    .copied()
                    .unwrap_or(i32::from(spec.key != "coordinates"))
                    .clamp(0, 1),
            );
        }
    });
    values[32] = game.settings.horizon.sky as i32;
    values[33] = game.settings.horizon.weather as i32;
    values[34] = game.settings.horizon.fog as i32;
    values[170] = game.settings.gui_scale as i32;
    values
}
fn sync_values(values: &[i32], on: bool) {
    let mut mask = u64::MAX & !(1 << 14);
    if on {
        for (offset, &bit) in model::EXTRA_BITS.iter().enumerate() {
            if bit >= 0 {
                if values[24 + offset] == 0 {
                    mask &= !(1 << bit);
                } else {
                    mask |= 1 << bit;
                }
            }
        }
    }
    let disabled: Vec<usize> = if on {
        values
            .iter()
            .skip(model::PARTICLE_START)
            .take(model::PARTICLE_NAMES.len())
            .enumerate()
            .filter_map(|(i, &v)| (v == 0).then_some(i))
            .collect()
    } else {
        Vec::new()
    };
    horizon_client_render::sodium_extra_filters::configure(mask, &disabled);
}
fn sync(game: &Game<'_>) {
    let on = game.native_on("sodium_horizon");
    SYNCED.with_borrow_mut(|last| {
        if *last != Some(on) {
            sync_values(&read(game), on);
            *last = Some(on);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_base_control_preserves_lighting_and_uses_valid_cloud_values() {
        let mut settings = Settings::default();
        settings.gamma = 0.37;
        settings.smooth_lighting = false;
        settings.biome_blend = 5;
        settings.extras.fullbright = true;
        settings.extras.dynamic_lights = true;
        settings.horizon.shadow_cache = true;
        settings.horizon.shader_pack = "lighting-test-pack".into();
        settings.horizon.hdr = true;
        for id in (0..24).chain([170]) {
            let now = if id == 170 {
                settings.gui_scale as i32
            } else {
                read_base(&settings)[id]
            };
            write(&mut settings, id, SPECS[id].cycle(now, false));
            assert_eq!(settings.gamma, 0.37);
            assert!(!settings.smooth_lighting);
            assert_eq!(settings.biome_blend, 5);
            assert!(settings.extras.fullbright);
            assert!(settings.extras.dynamic_lights);
            assert!(settings.horizon.shadow_cache);
            assert!(settings.horizon.hdr);
            assert_eq!(settings.horizon.shader_pack, "lighting-test-pack");
        }
        for (id, expected) in [(0, "false"), (1, "fast"), (2, "true")] {
            write(&mut settings, 6, id);
            assert_eq!(settings.clouds, expected);
            assert_eq!(Settings::parse(&settings.serialize()).clouds, expected);
        }
    }

    #[test]
    fn particle_controls_match_the_pinned_native_registry() {
        let types = horizon_client::particles::ParticleType::ALL;
        assert_eq!(types.len(), model::PARTICLE_NAMES.len());
        for (id, ty) in types.iter().enumerate() {
            assert_eq!(*ty as usize, id);
            assert_eq!(ty.name(), model::PARTICLE_NAMES[id]);
        }
    }
}
