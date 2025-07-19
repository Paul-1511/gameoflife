use std::time::{Duration, Instant};
use minifb::{Key, Window};

pub struct GameOfLife {
    width: usize,
    height: usize,
    current_state: Vec<Vec<bool>>,
    next_state: Vec<Vec<bool>>,
    last_update: Instant,
    frame_delay: Duration,
    paused: bool,
    generation: u64,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize, initial_delay_ms: u64) -> Self {
        GameOfLife {
            width,
            height,
            current_state: vec![vec![false; height]; width],
            next_state: vec![vec![false; height]; width],
            last_update: Instant::now(),
            frame_delay: Duration::from_millis(initial_delay_ms),
            paused: false,
            generation: 0,
        }
    }

    pub fn get_frame_delay(&self) -> Duration {
        self.frame_delay
    }

    pub fn update(&mut self) {
        if self.paused || self.last_update.elapsed() < self.frame_delay {
            return;
        }

        for x in 0..self.width {
            for y in 0..self.height {
                let live_neighbors = self.count_live_neighbors(x, y);
                let cell = self.current_state[x][y];
                
                self.next_state[x][y] = match (cell, live_neighbors) {
                    (true, 2) | (true, 3) => true,
                    (true, _) => false,
                    (false, 3) => true,
                    (false, _) => false,
                };
            }
        }

        std::mem::swap(&mut self.current_state, &mut self.next_state);
        self.last_update = Instant::now();
        self.generation += 1;
    }

    fn count_live_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                
                let nx = x as isize + i;
                let ny = y as isize + j;
                
                if nx >= 0 && ny >= 0 && nx < self.width as isize && ny < self.height as isize {
                    if self.current_state[nx as usize][ny as usize] {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn render_to_buffer(&self, buffer: &mut [u32], cell_size: usize) {
        for (i, pixel) in buffer.iter_mut().enumerate() {
            let x = (i % (self.width * cell_size)) / cell_size;
            let y = (i / (self.width * cell_size)) / cell_size;
            
            *pixel = if x < self.width && y < self.height && self.current_state[x][y] {
                0xFFFFFFFF // Blanco
            } else {
                0x00000000 // Negro
            };
        }
    }

    pub fn handle_input(&mut self, window: &Window) {
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            self.paused = !self.paused;
        }
        
        if window.is_key_down(Key::Up) {
            self.frame_delay += Duration::from_millis(10);
        }
        
        if window.is_key_down(Key::Down) && self.frame_delay > Duration::from_millis(10) {
            self.frame_delay -= Duration::from_millis(10);
        }
        
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            self.clear();
        }
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn set_cell(&mut self, x: usize, y: usize, state: bool) {
        if x < self.width && y < self.height {
            self.current_state[x][y] = state;
        }
    }

    pub fn get_cell_state(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            self.current_state[x][y]
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        for row in &mut self.current_state {
            for cell in row {
                *cell = false;
            }
        }
        self.generation = 0;
    }

    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        let mut new_current = vec![vec![false; new_height]; new_width];
        let new_next = vec![vec![false; new_height]; new_width];
        
        let min_width = usize::min(self.width, new_width);
        let min_height = usize::min(self.height, new_height);
        
        for x in 0..min_width {
            for y in 0..min_height {
                new_current[x][y] = self.current_state[x][y];
            }
        }
        
        self.width = new_width;
        self.height = new_height;
        self.current_state = new_current;
        self.next_state = new_next;
    }

    pub fn get_generation(&self) -> u64 {
        self.generation
    }
}