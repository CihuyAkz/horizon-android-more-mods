// SPDX-License-Identifier: MIT
fn extra_stars()->std::ops::ControlFlow<()> {
if !crate::sodium_extra_filters::enabled(15) {std::ops::ControlFlow::Break(())} else {std::ops::ControlFlow::Continue(())}
}
