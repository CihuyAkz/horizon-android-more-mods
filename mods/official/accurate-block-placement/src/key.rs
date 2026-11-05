// Android static adapter for Accurate Block Placement's key binding.
pub(super) fn run(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::android_bundled::accurate_block_placement as abp;
    if !pressed || !game.native_on(abp::ID) || !game.native_playing() {
        return;
    }
    let bound = game.native_mod_key(abp::ID, abp::KEY, abp::DEFAULT_KEY);
    if bound == abp::DEFAULT_KEY || key != bound {
        return;
    }
    let mut settings = {
        let game = &*game;
        abp::settings(|name| game.native_setting(abp::ID, name))
    };
    settings.enabled = !settings.enabled;
    abp::set_settings(settings);
    abp::reset();
    for (name, value) in settings.stored() {
        game.native_set_setting(abp::ID, name, &value);
    }
    game.native_chat(if settings.enabled { abp::ON } else { abp::OFF });
}
