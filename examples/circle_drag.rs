use raylib::prelude::*;

fn main(){
    let (mut rl, thread) = raylib::init()
    .size(640,480)
    .title("Test")
    .build();

    let circle_rad : f32 = 10.0;
    let mut circle_pos : Vector2 = Vector2::new(0.0,0.0);

    let mut mouse_pos: Vector2 = Vector2{x :2.0, y : 2.0};
    while !rl.window_should_close(){
        if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON){
            mouse_pos = rl.get_mouse_position();
        }

        let mut d = rl.begin_drawing(&thread);
        d.draw_circle_v(circle_pos, circle_rad, Color::BLACK);
        if check_collision_point_circle(mouse_pos,circle_pos, circle_rad) {
            circle_pos = mouse_pos;
        }
        d.clear_background(Color::WHITE);
    }
}
