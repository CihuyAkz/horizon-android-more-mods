// Android static adapter: sample the crosshair once per frame.
pub(super) fn run(game: &mut crate::game::Game<'_>) {
    use crate::game::android_bundled::accurate_block_placement as abp;
    let settings = {
        let game = &*game;
        abp::settings(|name| game.native_setting(abp::ID, name))
    };
    let Some(player) = game
        .player
        .as_ref()
        .map(|p| [p.position.x, p.position.y, p.position.z])
    else {
        abp::reset();
        return;
    };
    let allowed = settings.enabled
        && game.native_on(abp::ID)
        && (game.integrated.is_some() || settings.servers);
    let held = match game.held() {
        Some(stack) => Some(stack.clone()),
        None => game.offhand.clone(),
    };
    let block_item = held
        .as_ref()
        .and_then(|s| game.items().get(&s.item))
        .is_some_and(|i| i.block.is_some());
    let mut down = allowed
        && block_item
        && game.right
        && game.screen == crate::game::Screen::Playing
        && game.sleeping.is_none()
        && game.using.is_none();
    let mut target = None;
    if down && !game.entity_targeted() {
        if let Some(hit) = game.pick() {
            let state = game.world.block(hit.pos[0], hit.pos[1], hit.pos[2]);
            let scaffolding = held
                .as_ref()
                .is_some_and(|s| &*s.item == "minecraft:scaffolding")
                && &*game.res.facts().get(state).name == "minecraft:scaffolding";
            if scaffolding {
                down = false;
            } else {
                target = Some(abp::Sample {
                    pos: hit.pos,
                    face: hit.face,
                    point: hit.point,
                    replaceable: game.res.facts().get(state).replaceable,
                });
            }
        }
    }
    abp::frame(down, target, player, (game.yaw, game.pitch));
}
