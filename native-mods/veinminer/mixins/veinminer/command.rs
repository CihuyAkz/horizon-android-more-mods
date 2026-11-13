fn command_head(game: &mut crate::game::Game<'_>, command: &str) -> core::ops::ControlFlow<bool> {
    if !game.native_on("veinminer") {
        return core::ops::ControlFlow::Continue(());
    }
    match game.veinminer_command(command) {
        Some(result) => core::ops::ControlFlow::Break(result),
        None => core::ops::ControlFlow::Continue(()),
    }
}
