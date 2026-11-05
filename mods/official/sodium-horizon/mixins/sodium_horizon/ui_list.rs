// SPDX-License-Identifier: MIT
fn ui_list(mods:&mut Vec<crate::game::native_hooks::NativeMod>) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: "sodium_horizon".into(), title: "Sodium-style Video Settings 0.2.0".into(),
        description: "Tabbed video settings and non-lighting optimizations for Horizon.".into(),
        keys: vec![],
    });
}
