use minifb::{Window, WindowOptions, Scale, ScaleMode};
use crate::game::GameOfLife;

const ALIVE_COLOR: u32 = 0x00_FF_FF_FF;  // Cian
const DEAD_COLOR: u32 = 0x00_00_00_FF;   // Negro
const GRID_COLOR: u32 = 0x00_20_20_20;   // Gris oscuro

pub struct Renderer {
    window: Window,
    buffer: Vec<u32>,
    cell_size: usize,
    width: usize,
    height: usize,
    show_grid: bool,
}

impl Renderer {
    pub fn new(
        title: &str,
        grid_width: usize,
        grid_height: usize,
        cell_size: usize,
    ) -> Result<Self, String> {
        let width = grid_width * cell_size;
        let height = grid_height * cell_size;

        let mut window = Window::new(
            title,
            width,
            height,
            WindowOptions {
                resize: true,
                scale: Scale::X1,
                scale_mode: ScaleMode::AspectRatioStretch,
                ..WindowOptions::default()
            },
        ).map_err(|e| e.to_string())?;

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Ok(Self {
            window,
            buffer: vec![0; width * height],
            cell_size,
            width: grid_width,
            height: grid_height,
            show_grid: true,
        })
    }

    pub fn is_active(&mut self) -> bool { 
        self.window.is_active()
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn update(&mut self, game: &GameOfLife) {
        if !self.window.is_open() {
            return;
        }

        self.clear_buffer();
        self.draw_grid(game);
        
        let title = format!(
            "Conway's Game of Life | Gen: {} | Speed: {}ms {}",
            game.get_generation(),
            game.get_frame_delay().as_millis(),
            if game.is_paused() { "| PAUSED" } else { "" }
        );
        self.window.set_title(&title);

        self.window
            .update_with_buffer(&self.buffer, self.width * self.cell_size, self.height * self.cell_size)
            .unwrap_or_else(|e| eprintln!("Window update error: {}", e));
    }

    fn clear_buffer(&mut self) {
        for pixel in &mut self.buffer {
            *pixel = DEAD_COLOR;
        }
    }

    fn draw_grid(&mut self, game: &GameOfLife) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.draw_cell(x, y, game.get_cell_state(x, y));
            }
        }

        if self.show_grid && self.cell_size > 4 {
            self.draw_grid_lines();
        }
    }

    fn draw_cell(&mut self, x: usize, y: usize, alive: bool) {
        if x >= self.width || y >= self.height {
            return;
        }

        let color = if alive { ALIVE_COLOR } else { DEAD_COLOR };
        let px_start = x * self.cell_size;
        let py_start = y * self.cell_size;
        let cell_size = if self.show_grid { self.cell_size - 1 } else { self.cell_size };

        for px in px_start..px_start + cell_size {
            for py in py_start..py_start + cell_size {
                if px < self.width * self.cell_size && py < self.height * self.cell_size {
                    let index = py * (self.width * self.cell_size) + px;
                    if index < self.buffer.len() {
                        self.buffer[index] = color;
                    }
                }
            }
        }
    }

    fn draw_grid_lines(&mut self) {
        for x in 0..=self.width {
            let px = x * self.cell_size;
            for y in 0..self.height * self.cell_size {
                if px < self.width * self.cell_size && y < self.height * self.cell_size {
                    let index = y * (self.width * self.cell_size) + px;
                    if index < self.buffer.len() {
                        self.buffer[index] = GRID_COLOR;
                    }
                }
            }
        }

        for y in 0..=self.height {
            let py = y * self.cell_size;
            for x in 0..self.width * self.cell_size {
                if x < self.width * self.cell_size && py < self.height * self.cell_size {
                    let index = py * (self.width * self.cell_size) + x;
                    if index < self.buffer.len() {
                        self.buffer[index] = GRID_COLOR;
                    }
                }
            }
        }
    }

    pub fn should_close(&self) -> bool {
        !self.window.is_open()
    }

    pub fn handle_resize(&mut self, game: &mut GameOfLife) {
        let (width, height) = self.window.get_size();
        let new_grid_width = width / self.cell_size;
        let new_grid_height = height / self.cell_size;
        
        if new_grid_width != self.width || new_grid_height != self.height {
            self.width = new_grid_width;
            self.height = new_grid_height;
            self.buffer = vec![0; width * height];
            game.resize(new_grid_width, new_grid_height);
        }
    }
}