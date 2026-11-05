// SPDX-License-Identifier: MIT
// Original settings editor state; no Sodium code or assets.
#[derive(Clone, Copy)]
pub struct Spec {
    pub key: &'static str,
    pub title: &'static str,
    pub help: &'static str,
    pub tab: usize,
    pub min: i32,
    pub max: i32,
    pub step: i32,
    pub choices: &'static [&'static str],
}

macro_rules! option {
    ($key:literal, $title:literal, $help:literal, $tab:expr, $min:expr, $max:expr, $step:expr) => {
        Spec { key: $key, title: $title, help: $help, tab: $tab, min: $min, max: $max, step: $step, choices: &[] }
    };
    ($key:literal, $title:literal, $help:literal, $tab:expr, [$($choice:literal),+]) => {
        Spec { key: $key, title: $title, help: $help, tab: $tab, min: 0, max: [$( $choice ),+].len() as i32 - 1, step: 1, choices: &[$($choice),+] }
    };
}

pub const SPECS: &[Spec] = &[
    option!("render_distance", "Render Distance", "Terrain range in chunks. Higher distances increase mesh, CPU and GPU work.", 0, 2, 128, 1),
    option!("simulation_distance", "Simulation Distance", "Singleplayer simulation range in chunks. Servers control their own simulation range.", 0, 2, 128, 1),
    option!("max_fps", "Frame Rate Limit", "Maximum foreground FPS. 260 is unlimited. A cap can reduce GPU usage.", 0, 10, 260, 10),
    option!("vsync", "VSync", "Synchronize presentation to the display. With VSync on, FIFO presentation is used.", 0, ["OFF", "ON"]),
    option!("fullscreen", "Fullscreen", "Use the game's fullscreen display mode.", 0, ["OFF", "ON"]),
    option!("resolution_scale", "Render Scale", "World resolution as a percentage of the window. Lower values reduce GPU pixel work.", 0, 50, 200, 10),
    option!("clouds", "Clouds", "Cloud quality. Disabling clouds avoids their geometry and rendering cost.", 1, ["OFF", "Fast", "Fancy"]),
    option!("particles", "Particles", "Particle quantity: all, decreased or minimal. This uses Horizon's particle controls.", 1, ["All", "Decreased", "Minimal"]),
    option!("entity_distance", "Entity Distance", "Entity render distance in percent. Lower values reduce visible entity work.", 1, 50, 500, 25),
    option!("mipmap_levels", "Mipmap Levels", "Texture mip levels. Higher levels can reduce distant texture aliasing and bandwidth.", 1, 0, 4, 1),
    option!("animated_textures", "Animated Textures", "When on, Horizon updates animations used by visible terrain. OFF freezes block texture animation.", 1, ["OFF", "ON"]),
    option!("cutout_leaves", "Fast Leaves", "Render vanilla leaves without transparent cutouts. Changes foliage appearance.", 1, ["OFF", "ON"]),
    option!("cave_culling", "Block Face Culling", "Use section connectivity to skip terrain behind closed sections. Includes the optimized connectivity calculation.", 2, ["OFF", "ON"]),
    option!("entity_culling", "Entity Culling", "Skip entities hidden behind opaque terrain using Horizon's native occlusion culler.", 2, ["OFF", "ON"]),
    option!("leaf_culling", "Leaf Geometry", "OFF preserves fancy leaves. Solid modes simplify canopy interiors; Hollow removes shared leaf faces. Packs may force OFF.", 2, ["OFF", "Solid", "Aggressive", "Hollow"]),
    option!("chunk_updates", "Chunk Updates", "Maximum meshed sections uploaded per frame. Zero uploads all ready sections. A lower budget can smooth frame times.", 2, 0, 32, 1),
    option!("upload_budget", "Upload Budget", "Maximum mesh upload KiB per frame. Zero is unlimited. A small budget may delay terrain appearance.", 2, ["Unlimited", "1024 KiB", "2048 KiB", "4096 KiB", "8192 KiB", "16384 KiB", "32768 KiB"]),
    option!("workers", "Chunk Threads", "Worker count for chunk processing. Auto uses available cores. Takes effect after restarting Horizon.", 2, 0, 32, 1),
    option!("translucent_batching", "Translucent Batching", "Batch back-to-front translucent draws at the cost of more video memory. Takes effect after restart or resource reload.", 3, ["OFF", "ON"]),
    option!("present_mode", "Presentation Mode", "Auto, FIFO, Mailbox or Immediate where the driver supports it. VSync overrides this with FIFO.", 3, ["Auto", "FIFO", "Mailbox", "Immediate"]),
    option!("pacing", "Frame Pacing", "Efficient sleeps until the next frame. Precise finishes with a short CPU spin for tighter timing.", 3, ["Efficient", "Precise"]),
    option!("texture_budget", "Texture Budget", "Texture memory budget. Auto uses Horizon's choice. Applies after restart or resource reload.", 3, ["Auto", "128 MiB", "256 MiB", "512 MiB", "1024 MiB", "2048 MiB"]),
    option!("overlay", "Performance Overlay", "Show FPS, frame time, or a frame-time graph over the game.", 3, ["OFF", "FPS", "Frame Graph"]),
    option!("api", "Graphics API", "Select the native graphics backend. Unsupported backends can fail to start. Takes effect after restarting Horizon.", 3, ["Auto", "Vulkan", "DirectX 12", "OpenGL"]),
    option!("water_animation", "Water Animation", "Freeze animated water textures, avoiding their animation jobs.", 4, ["OFF", "ON"]),
    option!("lava_animation", "Lava Animation", "Freeze animated lava textures without changing emitted light or fluid simulation.", 4, ["OFF", "ON"]),
    option!("fire_animation", "Fire Animation", "Freeze block fire and soul-fire textures without changing fire simulation or light.", 4, ["OFF", "ON"]),
    option!("portal_animation", "Portal Animation", "Freeze animated portal block textures.", 4, ["OFF", "ON"]),
    option!("block_animation", "Other Block Animations", "Freeze other animated block and item atlas textures, including custom textures.", 4, ["OFF", "ON"]),
    option!("sculk_animation", "Sculk Animation", "Freeze animated sculk textures without changing block behavior.", 4, ["OFF", "ON"]),
    option!("all_particles", "All Particles", "Allow particle creation. OFF skips creation and later ticking/rendering of new particles.", 5, ["OFF", "ON"]),
    option!("mod_particles", "Mod Particles", "Allow named native-mod particles as a group. Vanilla types have individual switches below.", 5, ["OFF", "ON"]),
    option!("sky", "Sky", "Draw the native sky pass. This changes background rendering, never world lighting.", 6, ["OFF", "ON"]),
    option!("weather", "Rain and Snow", "Draw weather geometry. Weather simulation and server state continue unchanged.", 6, ["OFF", "ON"]),
    option!("fog", "Terrain Fog", "Draw terrain distance fog. Fluid and status-effect fog keep the game controls.", 6, ["OFF", "ON"]),
    option!("item_frames", "Item Frames", "Draw item-frame models and their contained items. Entities and interactions remain active.", 6, ["OFF", "ON"]),
    option!("armor_stands", "Armor Stands", "Draw world armor-stand models and equipment. Armor-stand behavior is unchanged.", 6, ["OFF", "ON"]),
    option!("paintings", "Paintings", "Draw painting geometry. Paintings still exist and can be interacted with.", 6, ["OFF", "ON"]),
    option!("advancement_toasts", "Advancement Toasts", "Allow advancement popups. OFF discards popups, never the advancements themselves.", 7, ["OFF", "ON"]),
    option!("recipe_toasts", "Recipe Toasts", "Allow recipe popups. Recipes still unlock when popups are disabled.", 7, ["OFF", "ON"]),
    option!("tutorial_toasts", "Tutorial Toasts", "Allow tutorial hint popups. Tutorial progression continues when they are hidden.", 7, ["OFF", "ON"]),
    option!("coordinates", "Coordinates", "Show a coordinate overlay while debug information is allowed. Does not bypass reduced-debug-info policy.", 7, ["OFF", "ON"]),
    option!("stars", "Stars", "Draw native stars. Turning this off skips the star draw without changing world lighting.", 6, ["OFF", "ON"]),
    option!("particle_block", "Block", "Allow new minecraft:block particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_block_crumble", "Block Crumble", "Allow new minecraft:block_crumble particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_trail", "Trail", "Allow new minecraft:trail particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_dust_pillar", "Dust Pillar", "Allow new minecraft:dust_pillar particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_falling_dust", "Falling Dust", "Allow new minecraft:falling_dust particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_smoke", "Smoke", "Allow new minecraft:smoke particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_large_smoke", "Large Smoke", "Allow new minecraft:large_smoke particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_white_smoke", "White Smoke", "Allow new minecraft:white_smoke particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_flame", "Flame", "Allow new minecraft:flame particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_soul_fire_flame", "Soul Fire Flame", "Allow new minecraft:soul_fire_flame particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_copper_fire_flame", "Copper Fire Flame", "Allow new minecraft:copper_fire_flame particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_small_flame", "Small Flame", "Allow new minecraft:small_flame particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_lava", "Lava", "Allow new minecraft:lava particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_dripping_water", "Dripping Water", "Allow new minecraft:dripping_water particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_falling_water", "Falling Water", "Allow new minecraft:falling_water particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_dripping_lava", "Dripping Lava", "Allow new minecraft:dripping_lava particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_falling_lava", "Falling Lava", "Allow new minecraft:falling_lava particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_landing_lava", "Landing Lava", "Allow new minecraft:landing_lava particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_dripping_honey", "Dripping Honey", "Allow new minecraft:dripping_honey particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_falling_honey", "Falling Honey", "Allow new minecraft:falling_honey particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_landing_honey", "Landing Honey", "Allow new minecraft:landing_honey particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_falling_nectar", "Falling Nectar", "Allow new minecraft:falling_nectar particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_falling_spore_blossom", "Falling Spore Blossom", "Allow new minecraft:falling_spore_blossom particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_spore_blossom_air", "Spore Blossom Air", "Allow new minecraft:spore_blossom_air particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_dripping_obsidian_tear", "Dripping Obsidian Tear", "Allow new minecraft:dripping_obsidian_tear particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_falling_obsidian_tear", "Falling Obsidian Tear", "Allow new minecraft:falling_obsidian_tear particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_landing_obsidian_tear", "Landing Obsidian Tear", "Allow new minecraft:landing_obsidian_tear particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_dripping_dripstone_water", "Dripping Dripstone Water", "Allow new minecraft:dripping_dripstone_water particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_falling_dripstone_water", "Falling Dripstone Water", "Allow new minecraft:falling_dripstone_water particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_dripping_dripstone_lava", "Dripping Dripstone Lava", "Allow new minecraft:dripping_dripstone_lava particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_falling_dripstone_lava", "Falling Dripstone Lava", "Allow new minecraft:falling_dripstone_lava particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_splash", "Splash", "Allow new minecraft:splash particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_rain", "Rain", "Allow new minecraft:rain particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_bubble", "Bubble", "Allow new minecraft:bubble particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_sulfur_bubbles", "Sulfur Bubbles", "Allow new minecraft:sulfur_bubbles particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_noxious_gas", "Noxious Gas", "Allow new minecraft:noxious_gas particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_noxious_gas_cloud", "Noxious Gas Cloud", "Allow new minecraft:noxious_gas_cloud particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_geyser", "Geyser", "Allow new minecraft:geyser particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_geyser_base", "Geyser Base", "Allow new minecraft:geyser_base particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_geyser_poof", "Geyser Poof", "Allow new minecraft:geyser_poof particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_geyser_plume", "Geyser Plume", "Allow new minecraft:geyser_plume particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_bubble_pop", "Bubble Pop", "Allow new minecraft:bubble_pop particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_bubble_column_up", "Bubble Column Up", "Allow new minecraft:bubble_column_up particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_current_down", "Current Down", "Allow new minecraft:current_down particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_underwater", "Underwater", "Allow new minecraft:underwater particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_portal", "Portal", "Allow new minecraft:portal particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_reverse_portal", "Reverse Portal", "Allow new minecraft:reverse_portal particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_enchant", "Enchant", "Allow new minecraft:enchant particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_nautilus", "Nautilus", "Allow new minecraft:nautilus particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_vault_connection", "Vault Connection", "Allow new minecraft:vault_connection particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_crit", "Crit", "Allow new minecraft:crit particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_enchanted_hit", "Enchanted Hit", "Allow new minecraft:enchanted_hit particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_damage_indicator", "Damage Indicator", "Allow new minecraft:damage_indicator particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_heart", "Heart", "Allow new minecraft:heart particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_angry_villager", "Angry Villager", "Allow new minecraft:angry_villager particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_happy_villager", "Happy Villager", "Allow new minecraft:happy_villager particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_composter", "Composter", "Allow new minecraft:composter particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_mycelium", "Mycelium", "Allow new minecraft:mycelium particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_dolphin", "Dolphin", "Allow new minecraft:dolphin particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_egg_crack", "Egg Crack", "Allow new minecraft:egg_crack particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_poof", "Poof", "Allow new minecraft:poof particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_explosion", "Explosion", "Allow new minecraft:explosion particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_explosion_emitter", "Explosion Emitter", "Allow new minecraft:explosion_emitter particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_campfire_cosy_smoke", "Campfire Cosy Smoke", "Allow new minecraft:campfire_cosy_smoke particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_campfire_signal_smoke", "Campfire Signal Smoke", "Allow new minecraft:campfire_signal_smoke particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_note", "Note", "Allow new minecraft:note particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_dust", "Dust", "Allow new minecraft:dust particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_totem_of_undying", "Totem Of Undying", "Allow new minecraft:totem_of_undying particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_firework", "Firework", "Allow new minecraft:firework particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_end_rod", "End Rod", "Allow new minecraft:end_rod particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_ash", "Ash", "Allow new minecraft:ash particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_white_ash", "White Ash", "Allow new minecraft:white_ash particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_crimson_spore", "Crimson Spore", "Allow new minecraft:crimson_spore particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_warped_spore", "Warped Spore", "Allow new minecraft:warped_spore particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_cherry_leaves", "Cherry Leaves", "Allow new minecraft:cherry_leaves particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_pale_oak_leaves", "Pale Oak Leaves", "Allow new minecraft:pale_oak_leaves particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_tinted_leaves", "Tinted Leaves", "Allow new minecraft:tinted_leaves particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_effect", "Effect", "Allow new minecraft:effect particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_instant_effect", "Instant Effect", "Allow new minecraft:instant_effect particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_entity_effect", "Entity Effect", "Allow new minecraft:entity_effect particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_witch", "Witch", "Allow new minecraft:witch particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_infested", "Infested", "Allow new minecraft:infested particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_cloud", "Cloud", "Allow new minecraft:cloud particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_sneeze", "Sneeze", "Allow new minecraft:sneeze particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_snowflake", "Snowflake", "Allow new minecraft:snowflake particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_soul", "Soul", "Allow new minecraft:soul particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_sculk_soul", "Sculk Soul", "Allow new minecraft:sculk_soul particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_dragon_breath", "Dragon Breath", "Allow new minecraft:dragon_breath particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_gust", "Gust", "Allow new minecraft:gust particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_small_gust", "Small Gust", "Allow new minecraft:small_gust particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_gust_emitter_large", "Gust Emitter Large", "Allow new minecraft:gust_emitter_large particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_gust_emitter_small", "Gust Emitter Small", "Allow new minecraft:gust_emitter_small particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_dust_color_transition", "Dust Color Transition", "Allow new minecraft:dust_color_transition particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_sculk_charge", "Sculk Charge", "Allow new minecraft:sculk_charge particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_sculk_charge_pop", "Sculk Charge Pop", "Allow new minecraft:sculk_charge_pop particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_shriek", "Shriek", "Allow new minecraft:shriek particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_sonic_boom", "Sonic Boom", "Allow new minecraft:sonic_boom particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_vibration", "Vibration", "Allow new minecraft:vibration particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_sweep_attack", "Sweep Attack", "Allow new minecraft:sweep_attack particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_glow", "Glow", "Allow new minecraft:glow particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_wax_on", "Wax On", "Allow new minecraft:wax_on particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_wax_off", "Wax Off", "Allow new minecraft:wax_off particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_electric_spark", "Electric Spark", "Allow new minecraft:electric_spark particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_scrape", "Scrape", "Allow new minecraft:scrape particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_squid_ink", "Squid Ink", "Allow new minecraft:squid_ink particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_glow_squid_ink", "Glow Squid Ink", "Allow new minecraft:glow_squid_ink particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_item", "Item", "Allow new minecraft:item particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_item_slime", "Item Slime", "Allow new minecraft:item_slime particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_item_snowball", "Item Snowball", "Allow new minecraft:item_snowball particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_item_cobweb", "Item Cobweb", "Allow new minecraft:item_cobweb particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_sulfur_cube_goo", "Sulfur Cube Goo", "Allow new minecraft:sulfur_cube_goo particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_flash", "Flash", "Allow new minecraft:flash particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_firefly", "Firefly", "Allow new minecraft:firefly particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_trial_spawner_detection", "Trial Spawner Detection", "Allow new minecraft:trial_spawner_detection particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_trial_spawner_detection_ominous", "Trial Spawner Detection Ominous", "Allow new minecraft:trial_spawner_detection_ominous particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_trial_omen", "Trial Omen", "Allow new minecraft:trial_omen particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_raid_omen", "Raid Omen", "Allow new minecraft:raid_omen particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_ominous_spawning", "Ominous Spawning", "Allow new minecraft:ominous_spawning particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_spit", "Spit", "Allow new minecraft:spit particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_fishing", "Fishing", "Allow new minecraft:fishing particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_dust_plume", "Dust Plume", "Allow new minecraft:dust_plume particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_red_poplar_leaves", "Red Poplar Leaves", "Allow new minecraft:red_poplar_leaves particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_orange_poplar_leaves", "Orange Poplar Leaves", "Allow new minecraft:orange_poplar_leaves particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_yellow_poplar_leaves", "Yellow Poplar Leaves", "Allow new minecraft:yellow_poplar_leaves particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_pause_mob_growth", "Pause Mob Growth", "Allow new minecraft:pause_mob_growth particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_reset_mob_growth", "Reset Mob Growth", "Allow new minecraft:reset_mob_growth particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("particle_block_marker", "Block Marker", "Allow new minecraft:block_marker particles. Existing particles expire normally after this is disabled.", 5, ["OFF", "ON"]),
    option!("gui_scale", "GUI Scale", "Interface scale. Zero selects Auto. Apply changes the interface size without changing world resolution.", 0, 0, 4, 1),
];

pub const TABS: [&str; 8] = [
    "General",
    "Quality",
    "Performance",
    "Advanced",
    "Animations",
    "Particles",
    "Details",
    "Extras",
];

impl Spec {
    pub fn label(&self, value: i32) -> String {
        if self.choices.is_empty() {
            value.to_string()
        } else {
            self.choices
                .get(value as usize)
                .map_or_else(|| format!("Custom ({value})"), |s| (*s).into())
        }
    }
    pub fn cycle(&self, value: i32, backwards: bool) -> i32 {
        if backwards {
            if value <= self.min {
                self.max
            } else {
                (value - self.step).max(self.min)
            }
        } else if value >= self.max {
            self.min
        } else {
            (value + self.step).min(self.max)
        }
    }
    pub fn slider(&self, fraction: f32) -> i32 {
        let steps = ((self.max - self.min) / self.step) as f32;
        self.min + (fraction.clamp(0.0, 1.0) * steps).round() as i32 * self.step
    }
}

pub struct Editor {
    pub original: Vec<i32>,
    pub draft: Vec<i32>,
    pub tab: usize,
    pub scroll: usize,
    pub focus: usize,
    pub dragging: Option<usize>,
    pub notice: bool,
    pub filter: String,
    pub searching: bool,
}

impl Editor {
    pub fn new(values: Vec<i32>) -> Self {
        Self {
            original: values.clone(),
            draft: values,
            tab: 0,
            scroll: 0,
            focus: 0,
            dragging: None,
            notice: false,
            filter: String::new(),
            searching: false,
        }
    }
    pub fn dirty(&self) -> bool {
        self.original != self.draft
    }
    pub fn changed(&self) -> Vec<usize> {
        self.original
            .iter()
            .zip(&self.draft)
            .enumerate()
            .filter_map(|(i, (a, b))| (a != b).then_some(i))
            .collect()
    }
    pub fn undo(&mut self) {
        self.draft.clone_from(&self.original);
    }
    pub fn commit(&mut self) {
        self.original.clone_from(&self.draft);
    }
    pub fn select_tab(&mut self, tab: usize) {
        self.tab = tab % TABS.len();
        self.scroll = 0;
        self.focus = 0;
        self.dragging = None;
        self.searching = false;
    }
    pub fn rows(&self) -> Vec<usize> {
        SPECS
            .iter()
            .enumerate()
            .filter_map(|(i, s)| {
                (s.tab == self.tab
                    && (self.tab != 5
                        || self.filter.is_empty()
                        || s.title
                            .to_ascii_lowercase()
                            .contains(&self.filter.to_ascii_lowercase())
                        || s.key.contains(&self.filter.to_ascii_lowercase())))
                .then_some(i)
            })
            .collect()
    }
}

#[derive(Clone, Copy)]
pub struct Layout {
    pub x: f32,
    pub width: f32,
    pub row_width: f32,
    pub top: f32,
    pub bottom: f32,
    pub visible: usize,
    pub wide: bool,
}

impl Layout {
    pub fn new(w: f32, h: f32) -> Self {
        let width = (w - 24.0).clamp(240.0, 780.0);
        let wide = width >= 480.0;
        let bottom = h - 67.0;
        Self {
            x: (w - width) * 0.5,
            width,
            row_width: if wide { width * 0.61 } else { width },
            top: 77.0,
            bottom,
            visible: ((bottom - 77.0) / 22.0).floor().max(1.0) as usize,
            wide,
        }
    }
    pub fn row(&self, slot: usize) -> [f32; 4] {
        [self.x, self.top + slot as f32 * 22.0, self.row_width, 20.0]
    }
    pub fn tab(&self, i: usize) -> [f32; 4] {
        [
            self.x + (i % 4) as f32 * self.width / 4.0,
            31.0 + (i / 4) as f32 * 20.0,
            self.width / 4.0 - 2.0,
            18.0,
        ]
    }
    pub fn button(&self, i: usize) -> [f32; 4] {
        [
            self.x + i as f32 * self.width / 4.0,
            self.bottom + 39.0,
            self.width / 4.0 - 3.0,
            20.0,
        ]
    }
}

pub fn hit(at: [f32; 2], rect: [f32; 4]) -> bool {
    at[0] >= rect[0] && at[0] < rect[0] + rect[2] && at[1] >= rect[1] && at[1] < rect[1] + rect[3]
}

pub const EXTRA_BITS: &[i32] = &[
    0, 1, 2, 3, 4, 5, 6, 7, -1, -1, -1, 8, 9, 10, 11, 12, 13, 14, 15,
];
pub const PARTICLE_START: usize = 43;
pub const PARTICLE_NAMES: &[&str] = &[
    "block",
    "block_crumble",
    "trail",
    "dust_pillar",
    "falling_dust",
    "smoke",
    "large_smoke",
    "white_smoke",
    "flame",
    "soul_fire_flame",
    "copper_fire_flame",
    "small_flame",
    "lava",
    "dripping_water",
    "falling_water",
    "dripping_lava",
    "falling_lava",
    "landing_lava",
    "dripping_honey",
    "falling_honey",
    "landing_honey",
    "falling_nectar",
    "falling_spore_blossom",
    "spore_blossom_air",
    "dripping_obsidian_tear",
    "falling_obsidian_tear",
    "landing_obsidian_tear",
    "dripping_dripstone_water",
    "falling_dripstone_water",
    "dripping_dripstone_lava",
    "falling_dripstone_lava",
    "splash",
    "rain",
    "bubble",
    "sulfur_bubbles",
    "noxious_gas",
    "noxious_gas_cloud",
    "geyser",
    "geyser_base",
    "geyser_poof",
    "geyser_plume",
    "bubble_pop",
    "bubble_column_up",
    "current_down",
    "underwater",
    "portal",
    "reverse_portal",
    "enchant",
    "nautilus",
    "vault_connection",
    "crit",
    "enchanted_hit",
    "damage_indicator",
    "heart",
    "angry_villager",
    "happy_villager",
    "composter",
    "mycelium",
    "dolphin",
    "egg_crack",
    "poof",
    "explosion",
    "explosion_emitter",
    "campfire_cosy_smoke",
    "campfire_signal_smoke",
    "note",
    "dust",
    "totem_of_undying",
    "firework",
    "end_rod",
    "ash",
    "white_ash",
    "crimson_spore",
    "warped_spore",
    "cherry_leaves",
    "pale_oak_leaves",
    "tinted_leaves",
    "effect",
    "instant_effect",
    "entity_effect",
    "witch",
    "infested",
    "cloud",
    "sneeze",
    "snowflake",
    "soul",
    "sculk_soul",
    "dragon_breath",
    "gust",
    "small_gust",
    "gust_emitter_large",
    "gust_emitter_small",
    "dust_color_transition",
    "sculk_charge",
    "sculk_charge_pop",
    "shriek",
    "sonic_boom",
    "vibration",
    "sweep_attack",
    "glow",
    "wax_on",
    "wax_off",
    "electric_spark",
    "scrape",
    "squid_ink",
    "glow_squid_ink",
    "item",
    "item_slime",
    "item_snowball",
    "item_cobweb",
    "sulfur_cube_goo",
    "flash",
    "firefly",
    "trial_spawner_detection",
    "trial_spawner_detection_ominous",
    "trial_omen",
    "raid_omen",
    "ominous_spawning",
    "spit",
    "fishing",
    "dust_plume",
    "red_poplar_leaves",
    "orange_poplar_leaves",
    "yellow_poplar_leaves",
    "pause_mob_growth",
    "reset_mob_growth",
    "block_marker",
];
