use sdl2::render::WindowCanvas;
use sdl2::VideoSubsystem;

pub struct Window {
    canvas: WindowCanvas,
}

impl Window {
    pub fn create_window(video_subsystem: &VideoSubsystem, title: &str, width: u32, height: u32) -> Result<Self, String> {
        let window = video_subsystem.window(title, width, height)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");

        let canvas = window.into_canvas().build()
            .expect("could not make a canvas");

        Ok(Window {
            canvas,
        })
    }

    pub fn render(&mut self) {
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn is_close_requested(&self) -> bool {
        false 
    }

    pub fn get_width(&self) -> u32 {
        self.canvas.window().size().0
    }

    pub fn get_height(&self) -> u32 {
        self.canvas.window().size().1
    }

    pub fn get_title(&self) -> String {
        self.canvas.window().title().to_string()
    }
}
