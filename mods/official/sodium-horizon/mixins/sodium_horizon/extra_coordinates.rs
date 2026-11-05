// SPDX-License-Identifier: MIT
fn extra_coordinates(game:&mut crate::game::Game<'_>,batch:&mut HudBatch,corners:&mut Corners)->() {

    use horizon_client::extras::Corner;
    if !game.native_on("sodium_horizon") || !horizon_client_render::sodium_extra_filters::enabled(14)
        || game.reduced_debug_info() || !game.native_corner_shown(Corner::TopLeft) {return;}
    let Some(player)=game.native_player() else{return};
    let line=format!("X: {:.1}  Y: {:.1}  Z: {:.1}",player.x,player.y,player.z);
    let width=game.native_text_width(&line)+8.0;
    let (x,y)=corners.place(Corner::TopLeft,width,16.0);
    game.native_fill(batch,[x,y,width,16.0],0xA010141A);
    game.native_text(batch,&line,[x+4.0,y+4.0],0xE8EBEF,true);

}
