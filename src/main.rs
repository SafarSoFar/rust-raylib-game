use raylib::prelude::*;
//use raylib::ffi;
fn main(){
    let mut isButtonActivated : bool = false;
    let (mut rl, thread) = raylib::init()
        .size(640,480)
        .title("Test")
        .build();

    let mut mousePos: Vector2 = Vector2{x :2.0, y : 2.0};
    while !rl.window_should_close(){
        if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON){
            mousePos = rl.get_mouse_position();
        }

        let mut d = rl.begin_drawing(&thread);
        d.draw_circle_v(mousePos, 5.0, Color::BLACK);
        d.clear_background(Color::WHITE);
    }
}