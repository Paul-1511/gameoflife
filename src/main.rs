mod game;
mod patterns;
mod render;

use crate::game::GameOfLife;
use crate::patterns::PatternInitializer;

const INITIAL_WIDTH: usize = 100;
const INITIAL_HEIGHT: usize = 100;
const CELL_SIZE: usize = 8;
const INITIAL_DELAY_MS: u64 = 100;

fn main() -> Result<(), String> {
    let mut renderer = render::Renderer::new(
        "Conway's Game of Life",
        INITIAL_WIDTH,
        INITIAL_HEIGHT,
        CELL_SIZE,
    )?;

    let mut game = GameOfLife::new(INITIAL_WIDTH, INITIAL_HEIGHT, INITIAL_DELAY_MS);
    PatternInitializer::initialize_patterns(&mut game);

    while !renderer.should_close() {
        renderer.handle_resize(&mut game);
        
        if renderer.is_active() {
            game.handle_input(renderer.get_window());
            
            if !game.is_paused() {
                game.update();
            }
        }

        renderer.update(&game);
    }

    Ok(())
}