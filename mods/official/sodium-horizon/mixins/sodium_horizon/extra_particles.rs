// SPDX-License-Identifier: MIT
fn extra_particles(options:&ParticleOptions)->std::ops::ControlFlow<()> {

    if !crate::sodium_extra_filters::particle_allowed(options.ty as usize, options.mod_particle.is_some()) {
        std::ops::ControlFlow::Break(())
    } else {std::ops::ControlFlow::Continue(())}

}
