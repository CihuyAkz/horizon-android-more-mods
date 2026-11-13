// A chat line with a shared place: add the map's clickable [Add] action.
fn official_worldmap_chat(
    mut message: horizon_client::text::Text,
    game: &crate::game::Game<'_>,
) -> horizon_client::text::Text {
    crate::game::official_worldmap::chat_received(game, &mut message);
    message
}
