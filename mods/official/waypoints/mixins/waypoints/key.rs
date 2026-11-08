// Android: N creates at the current block; M toggles waypoint labels.
// Full waypoint management is available on Options > Mods > Waypoints.
fn waypoints_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::waypoints_mod as m;
    if !pressed || !game.native_on(m::ID) || !game.native_playing() {
        return;
    }
    let create = game.native_mod_key(m::ID, m::KEY_CREATE, m::DEFAULT_CREATE);
    let list = game.native_mod_key(m::ID, m::KEY_LIST, m::DEFAULT_LIST);

    if key == create {
        let Some(player) = game.native_player() else {
            return;
        };
        let here = m::Here {
            pos: [player.x, player.y, player.z],
            dimension: player.dimension.to_owned(),
        };
        m::with(|state| state.create_at(&here));
    } else if key == list {
        let stored = game.native_setting(m::ID, "labels");
        let enabled = m::with(|state| {
            let current = *state.labels.get_or_insert_with(|| m::read_on(stored));
            let next = !current;
            state.labels = Some(next);
            next
        });
        game.native_set_setting(m::ID, "labels", if enabled { "true" } else { "false" });
    }
}
