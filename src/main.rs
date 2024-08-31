mod window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use window::Window;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut my_window = Window::create_window(&video_subsystem, "game tutorial", 800, 600)?;

    println!("Window title: {}", my_window.get_title());
    println!("Window width: {}", my_window.get_width());
    println!("Window height: {}", my_window.get_height());

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        if my_window.is_close_requested() {
            break 'running;
        }

        my_window.render();
        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
