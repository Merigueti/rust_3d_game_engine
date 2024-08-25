use sdl2::render::WindowCanvas;
use sdl2::VideoSubsystem; // Importando diretamente de sdl2

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
}
