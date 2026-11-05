// SPDX-License-Identifier: MIT
fn section_visibility(pad: &Padded, info: &[Info], stack: &mut Vec<usize>) -> u64 {
    let mut open = [0u16; 256];
    for y in 0..16 {
        for z in 0..16 {
            let offset = pi(0, y, z);
            let row = &mut open[(y * 16 + z) as usize];
            for x in 0..16 {
                if !info[pad.blocks[offset + x] as usize].solid {
                    *row |= 1 << x;
                }
            }
        }
    }
    crate::sodium_horizon_visibility::resolve(&open, stack)
}
