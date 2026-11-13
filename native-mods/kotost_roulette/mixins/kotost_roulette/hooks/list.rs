fn kotost_roulette_list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: "kotost_roulette".into(),
        title: "Kotost Roulette 0.1.0".into(),
        description: "A server-side item roulette minigame. Use /roulette to open the wheel; /roulette give gives you the Roulette item.".into(),
        keys: Vec::new(),
    });
}
