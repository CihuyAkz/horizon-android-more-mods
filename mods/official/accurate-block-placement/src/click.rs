// Android static adapter: persist a click from the settings page.
pub(super) fn run(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::android_bundled::accurate_block_placement as abp;
    if mod_id != abp::ID {
        return;
    }
    let mut settings = {
        let game = &*game;
        abp::settings(|name| game.native_setting(abp::ID, name))
    };
    settings.click(button);
    abp::set_settings(settings);
    abp::reset();
    for (name, value) in settings.stored() {
        game.native_set_setting(abp::ID, name, &value);
    }
}
