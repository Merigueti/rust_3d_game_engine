use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;

pub struct Input {
    current_keys: HashSet<Keycode>,    // Armazena as teclas atualmente pressionadas
    previous_keys: HashSet<Keycode>,   // Armazena as teclas pressionadas no frame anterior
    event_pump: sdl2::EventPump
}

impl Input {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        Input {
            current_keys: HashSet::new(),
            previous_keys: HashSet::new(),
            event_pump: sdl_context.event_pump().unwrap(),
        }
    }

    // Atualiza o estado das teclas, copiando o estado atual para 'previous_keys'
    // e atualizando 'current_keys' com o novo estado das teclas
    pub fn update(&mut self) {
        self.previous_keys = self.current_keys.clone(); // Salva o estado anterior
        self.current_keys.clear(); // Limpa as teclas atuais para atualizar o estado

        // Verifica todos os eventos de teclado
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(key), .. } => {
                    self.current_keys.insert(key);
                }
                Event::KeyUp { keycode: Some(key), .. } => {
                    self.current_keys.remove(&key);
                }
                _ => {}
            }
        }
    }

    // Verifica se a tecla especificada está sendo pressionada no momento
    pub fn get_key(&self, key_code: Keycode) -> bool {
        self.current_keys.contains(&key_code)
    }

    // Verifica se a tecla foi pressionada no frame atual (mudança de estado de não pressionada para pressionada)
    pub fn get_key_down(&self, key_code: Keycode) -> bool {
        self.current_keys.contains(&key_code) && !self.previous_keys.contains(&key_code)
    }

    pub fn get_key_up(&self, key_code: Keycode) -> bool {
        !self.current_keys.contains(&key_code) && self.previous_keys.contains(&key_code)
    }
}