// Android native Options > Mods click handler.
fn waypoints_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::waypoints_mod as m;
    if mod_id != m::ID {
        return;
    }

    if button <= 1 {
        let name = if button == 0 { "labels" } else { "deaths" };
        let stored = game.native_setting(m::ID, name);
        let on = m::with(|state| {
            let slot = if button == 0 { &mut state.labels } else { &mut state.deaths };
            let next = !slot.unwrap_or_else(|| m::read_on(stored));
            *slot = Some(next);
            next
        });
        game.native_set_setting(m::ID, name, if on { "true" } else { "false" });
        return;
    }

    if button == 2 {
        if let Some(player) = game.native_player() {
            let here = m::Here {
                pos: [player.x, player.y, player.z],
                dimension: player.dimension.to_owned(),
            };
            m::with(|state| state.create_at(&here));
        }
        return;
    }

    let offset = button - 3;
    let index = offset / 2;
    let delete = offset % 2 == 1;
    m::with(|state| {
        let action = if delete {
            m::Action::Delete(index)
        } else {
            m::Action::Toggle(index)
        };
        state.act(action, None);
    });
}
