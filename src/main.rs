mod window;
mod time;
mod game;
mod input;
use window::Window;
use game::Game;
use time::{Time, SECOND};
use input::{Input};
use sdl2::keyboard::Keycode;


fn main() -> Result<(), String> {
    let game: Game = Game::new_game();
    const TITLE: &str = "My Game Engine";
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    const FRAME_CAP: f64 = 5000.0;

    let is_running: bool = true;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut my_window = Window::create_window(&video_subsystem, TITLE, WIDTH, HEIGHT)?;
    let mut input = Input::new(&sdl_context);

    println!("Window title: {}", my_window.get_title());
    println!("Window width: {}", my_window.get_width());
    println!("Window height: {}", my_window.get_height());

    let mut frames:u128 = 0;
    let mut frame_counter:u128 = 0;

    let frame_time: f64 = 1.0 / FRAME_CAP;

    let mut last_time: u128 = Time::get_time();
    let mut unprocessed_time: f64 = 0.0;

    'running: loop {
        let mut render: bool = false;
        let start_time: u128 = Time::get_time();
        let passed_time: u128 = start_time - last_time;

        last_time = start_time;
        unprocessed_time += (passed_time as f64) / (SECOND as f64);
        frame_counter += passed_time;
        while unprocessed_time > frame_time {
            
            render = true;
            unprocessed_time -= frame_time;
            
            input.update();
            game.input();
            game.update();
            game.render();
            
            if input.get_key(Keycode::Escape) {
                println!("Escape key pressed, exiting...");
                break 'running;
            }
            if input.get_key_down(Keycode::A) {
                println!("key_down");
            }
            if input.get_key_up(Keycode::A) {
                println!("key_up");
            }

            if my_window.is_close_requested() || is_running == false {
                break 'running;
            }

            if frame_counter >= SECOND{
                println!("frames = {frames}");
                println!("frame_counter = {frame_counter}");
                frames = 0;
                frame_counter = 0;
            }

        }
        if render{
            my_window.render();
            frames += 1;
        }
    }
    
    Ok(())
}
