use std::time::{Duration, SystemTime};

use raylib::prelude::*;

fn apply_gravity(vec : &mut Vector2){
    vec.y += 0.1;
}

fn calculate_velocity(prev_frame : (SystemTime, Vector2), cur_frame : (SystemTime, Vector2)) -> f32{
    let distance : f32 = f32::powf(cur_frame.1.x - prev_frame.1.x,2.0) - f32::powf(cur_frame.1.y - prev_frame.1.y,2.0);   
    println!("{}", distance); 
    return distance / cur_frame.0.duration_since(prev_frame.0).unwrap().as_secs_f32(); 
}

fn main(){

    let start = SystemTime::now();

    const W_WIDTH : i32 = 640;
    const W_HEIGHT: i32 = 480;

    let (mut rl, thread) = raylib::init()
    .size(W_WIDTH,W_HEIGHT)
    .title("Test")
    .build();

    let circle_rad : f32 = 10.0;
    let mut circle_pos : Vector2 = Vector2::new(50.0,50.0);


    let mut prev_mouse_pos : Vector2 = Vector2::new(0.0,0.0);
    let mut prev_mouse_frame_time : SystemTime;

    let mut mouse_pos: Vector2 = Vector2{x :2.0, y : 2.0};
    let mut mouse_frame_time : SystemTime = SystemTime::now();


    while !rl.window_should_close(){
        if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON){
        }
        else{
            if circle_pos.y < W_HEIGHT as f32 -circle_rad{
                apply_gravity(&mut circle_pos);
            }
            
        }
        prev_mouse_pos = mouse_pos;
        prev_mouse_frame_time = mouse_frame_time;
        mouse_pos = rl.get_mouse_position();
        mouse_frame_time = SystemTime::now();
        calculate_velocity((prev_mouse_frame_time, prev_mouse_pos), (mouse_frame_time, mouse_pos));

        let mut d = rl.begin_drawing(&thread);
        d.draw_circle_v(circle_pos, circle_rad, Color::BLACK);
        if check_collision_point_circle(mouse_pos,circle_pos, circle_rad) {
            if mouse_pos.x < W_WIDTH as f32-circle_rad && mouse_pos.y < W_HEIGHT as f32-circle_rad
            && mouse_pos.x-circle_rad > 0.0 && mouse_pos.y-circle_rad > 0.0 {
                circle_pos = mouse_pos;
            }
        }
        d.clear_background(Color::WHITE);
    }
}
