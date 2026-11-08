// Android native Options > Mods page: visibility/death toggles, quick create,
// and per-waypoint visibility/delete controls.
fn waypoints_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::waypoints_mod as m;
    if mod_id != m::ID {
        return;
    }
    let labels = game.native_setting(m::ID, "labels");
    let deaths = game.native_setting(m::ID, "deaths");
    m::with(|state| {
        let labels = *state.labels.get_or_insert_with(|| m::read_on(labels));
        let deaths = *state.deaths.get_or_insert_with(|| m::read_on(deaths));
        buttons.push(m::labels_label(labels));
        buttons.push(m::deaths_label(deaths));
        buttons.push("Create waypoint at my position".to_owned());

        for (index, waypoint) in state.waypoints.iter().enumerate() {
            let visibility = if waypoint.hidden { "Show" } else { "Hide" };
            buttons.push(format!("{visibility}: {}", waypoint.name));
            let delete = if state.confirm_delete == Some(index) {
                format!("Confirm delete: {}", waypoint.name)
            } else {
                format!("Delete: {}", waypoint.name)
            };
            buttons.push(delete);
        }
    });
}
