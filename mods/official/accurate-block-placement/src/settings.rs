// Android static adapter: the mod's Options > Mods settings page.
pub(super) fn run(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::android_bundled::accurate_block_placement as abp;
    if mod_id != abp::ID {
        return;
    }
    let game = &*game;
    let settings = abp::settings(|name| game.native_setting(abp::ID, name));
    buttons.extend(settings.labels());
}
