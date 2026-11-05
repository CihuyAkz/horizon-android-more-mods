// SPDX-License-Identifier: MIT
fn extra_sprite(sprites:&Sprites,name:&str)->() {

    if let Ok(layer)=u32::try_from(sprites.native.len()) {crate::sodium_extra_filters::register_layer(layer,name);}

}
