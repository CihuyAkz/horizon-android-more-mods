// Once a frame: load this world's waypoints and track death markers.
// Horizon Android exposes native mod settings, not a generic native mod screen.
fn waypoints_frame(game: &mut crate::game::Game<'_>) {
    use crate::game::waypoints_mod as m;
    let key = if game.native_player().is_none() {
        None
    } else if let (None, Some(remote)) = (&game.integrated, &game.remote) {
        Some(format!("server-{}", remote.session.address.trim().to_lowercase()))
    } else if let Some(folder) = game
        .world_icon
        .as_ref()
        .and_then(|icon| icon.path.parent()?.file_name())
    {
        Some(format!("world-{}", folder.to_string_lossy()))
    } else {
        let current = m::with(|state| state.world.clone())
            .filter(|world| world.starts_with("world-"));
        let named = game.integrated.as_ref().map(|i| format!("world-{}", i.world_name));
        current.or(named).or_else(|| Some("world".into()))
    };

    let dir = game.native_data_dir(m::ID);
    let (load_labels, load_deaths) =
        m::with(|state| (state.labels.is_none(), state.deaths.is_none()));
    let stored_labels = load_labels
        .then(|| game.native_setting(m::ID, "labels"))
        .flatten();
    let stored_deaths = load_deaths
        .then(|| game.native_setting(m::ID, "deaths"))
        .flatten();
    let enabled = game.native_on(m::ID);
    let dead = game.screen == crate::game::Screen::Dead;
    let here = game.native_player().map(|player| m::Here {
        pos: [player.x, player.y, player.z],
        dimension: player.dimension.to_owned(),
    });
    let known = game.level.last_death.as_ref();

    m::with(|state| {
        state.load_world(key, dir);
        state.labels.get_or_insert_with(|| m::read_on(stored_labels));
        let deaths = *state.deaths.get_or_insert_with(|| m::read_on(stored_deaths));
        state.track_death(enabled && deaths, dead, known, here.as_ref());
    });
}
