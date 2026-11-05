// Adapted from Horizon (MIT), copyright (c) 2026 Ludovic Marie and the Horizon contributors.
// Change: skip disabled animation categories before computing blends or queueing jobs.
// SPDX-License-Identifier: MIT
fn extra_animations(animator:&mut SpriteAnimator,tick:u64,active:Option<&[u64]>)->std::ops::ControlFlow<usize> {
let result={
        let mut count = 0;
        animator.changed.clear();
        for sprite in &mut animator.sprites {
            if !crate::sodium_extra_filters::animation_allowed(sprite.animation.layer) {continue;}
            if let Some(active) = active {
                let layer = sprite.animation.layer as usize;
                let word = layer / u64::BITS as usize;
                let bit = 1_u64 << (layer % u64::BITS as usize);
                if active.get(word).is_none_or(|active| active & bit == 0) {
                    continue;
                }
            }
            let blend = sprite.animation.blend(tick);
            if sprite.shown == Some(blend) {
                continue;
            }
            sprite.shown = Some(blend);
            animator.changed.push(sprite.animation.layer);
            let job = [
                sprite.layer,
                sprite.frames[blend.frame],
                sprite.frames[blend.next],
                blend.millis,
                sprite.edge,
            ];
            let jobs = &mut animator.groups[sprite.group].jobs;
            for word in job {
                jobs.extend_from_slice(&word.to_le_bytes());
            }
            count += 1;
        }
        count
    };
std::ops::ControlFlow::Break(result)
}
