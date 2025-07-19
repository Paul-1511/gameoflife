use super::game::GameOfLife;
use rand::random;

pub trait PatternInitializer {
    fn initialize_patterns(&mut self);
}

impl PatternInitializer for GameOfLife {
    fn initialize_patterns(&mut self) {
        // Clear any existing pattern
        self.clear();

        // Pulsar (oscillator, period 3) at (10, 10)
        self.set_pattern(10, 10, &[
            "....##....",
            "...#..#...",
            "..#....#..",
            ".#....#...",
            "#....#....",
            "##..##....",
            "....#....#",
            "....#....#",
            "....##..##",
            "....#....#",
        ]);

        // Block (still life) at (20, 15)
        self.set_pattern(20, 15, &[
            ".##.",
            "#..#",
            ".#.#",
            "..#."
        ]);

        // Glider (spaceship, period 4) at (15, 20)
        self.set_pattern(15, 20, &[
            " # ",
            "  #",
            "###",
        ]);

        // Random ring in (5..25, 5..25) with 20% density for potential evolution
        for x in 5..25 {
            for y in 5..25 {
                if ((x as i32 - 15).pow(2) + (y as i32 - 15).pow(2)) <= 100 && random::<f64>() < 0.4 {
                    self.set_cell(x, y, true);
                }
            }
        }

        // Gosper Glider Gun (infinite growth) at (5, 40)
        self.set_pattern(5, 40, &[
            "........................#...........",
            "......................#.#...........",
            "............##......##............##",
            "...........#...#....##............##",
            "##........#.....#...##..............",
            "##........#...#.##....#.#...........",
            "..........#.....#.......#...........",
            "...........#...#....................",
            "............##......................",
        ]);

        // Pentadecathlon (oscillator, period 15) at (35, 10)
        self.set_pattern(35, 10, &[
            "##",
            "##",
            "##",
            "##",
            "##",
            "##",
            "##",
            "##",
            "##",
            "##",
        ]);

        // Beacon (oscillator, period 2) at (30, 25)
        self.set_pattern(30, 25, &[
            "##..",
            "#...",
            "...#",
            "..##",
        ]);

        // Toad (oscillator, period 2) at (40, 20)
        self.set_pattern(40, 20, &[
            ".###",
            "###.",
        ]);

        // Lightweight Spaceship (LWSS, period 4) at (50, 15)
        self.set_pattern(50, 15, &[
            "#..#.",
            "....#",
            "#...#",
            ".###.",
        ]);

        // Acorn (methuselah, long-lived) at (45, 30)
        self.set_pattern(45, 30, &[
            ".#.....",
            "...#...",
            "##..###",
        ]);

        // Diehard (methuselah, lasts 130 generations) at (25, 35)
        self.set_pattern(25, 35, &[
            "......#.",
            "##......",
            ".#...###",
        ]);

        // R-pentomino (methuselah, very long-lived) at (60, 10)
        self.set_pattern(60, 10, &[
            ".##",
            "##.",
            ".#.",
        ]);

        // Blinker (oscillator, period 2) at (55, 25)
        self.set_pattern(55, 25, &[
            "###",
        ]);

        // Boat (still life) at (65, 20)
        self.set_pattern(65, 20, &[
            "##.",
            "#.#",
            ".#.",
        ]);

        // Larger random cluster in (45..65, 45..65) with 15% density
        for x in 45..65 {
            for y in 45..65 {
                if random::<f64>() < 0.30 {
                    self.set_cell(x, y, true);
                }
            }
        }
    }
}

impl GameOfLife {
    pub fn set_pattern(&mut self, x_offset: usize, y_offset: usize, pattern: &[&str]) {
        for (y, line) in pattern.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    self.set_cell(x + x_offset, y + y_offset, true);
                }
            }
        }
    }
}