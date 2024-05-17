use crate::entities::entity::{Drawable, EntityFuncs, Movable};
use crate::entities::player::Player;
use crate::util::{lerp_color, map_noise};
use bracket_lib::color::{BLACK, RGB, WHITE};
use bracket_lib::terminal::{BTerm, GameState, Point, VirtualKeyCode};

// MARK: {Cell}
#[derive(Clone, Debug)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub fg: RGB,
    pub bg: RGB,
    pub char: char,
}

// MARK: impl {Cell}
impl Cell {
    pub fn new(x: i32, y: i32, c: char, fg: RGB, bg: RGB) -> Self {
        Self {
            x,
            y,
            fg,
            bg,
            char: c,
        }
    }
}

// MARK: #EntityFuncs for {Cell}
impl EntityFuncs for Cell {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn char(&self) -> char {
        self.char
    }

    fn fg_color(&self) -> RGB {
        self.fg
    }

    fn bg_color(&self) -> RGB {
        self.bg
    }

    fn set_x(&mut self, x: i32) {
        self.x = x
    }

    fn set_y(&mut self, y: i32) {
        self.y = y
    }

    fn set_pos(&mut self, x: i32, y: i32) {
        self.set_x(x);
        self.set_y(y);
    }

    fn set_char(&mut self, c: char) {
        self.char = c
    }

    fn set_fg_color(&mut self, c: RGB) {
        self.fg = c
    }

    fn set_bg_color(&mut self, c: RGB) {
        self.bg = c
    }
}

// MARK: {Map}
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub grid: Vec<Vec<Cell>>,
    pub seed: u64,
    pub col_val: f32,
}

// MARK: impl {Map}
impl Map {
    pub fn new(width: u32, height: u32, seed: u64, col_val: f32) -> Self {
        Self {
            width,
            height,
            grid: Vec::new(),
            seed,
            col_val,
        }
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Cell {
        self.grid
            .get(y as usize)
            .expect("couldn't get cell / y index out of bounds")
            .get(x as usize)
            .expect("couldn't get cell / x index out of bounds")
            .clone()
    }

    // MARK: impl {Map} -> generate
    pub fn generate(&mut self, seed: u64) {
        let noise = map_noise(seed);

        let mut cells = vec![
            vec![
                Cell::new(0, 0, '█', RGB::named(WHITE), RGB::named(BLACK));
                self.width as usize
            ];
            self.height as usize
        ];

        for x in 0..self.width {
            for y in 0..self.height {
                let mut fg_color;
                let noise = noise.get_noise(x as f32 / 15.0, y as f32 / 15.0);
                let noise = (noise + 1.0) / 2.0;

                fg_color = RGB::from_u8(0, (noise * self.col_val) as u8, 0);
                fg_color = lerp_color(RGB::from_u8(0, 48, 255), fg_color, noise * 1.3, true);

                if fg_color.g >= fg_color.b {
                    fg_color = lerp_color(
                        RGB::from_u8(16, 48, 0),
                        RGB::from_u8(48, 200, 8),
                        noise * 0.8,
                        true,
                    );
                } else {
                    fg_color = lerp_color(
                        RGB::from_u8(0, 64, 200),
                        RGB::from_u8(0, 0, 48),
                        noise * 0.8,
                        true,
                    );
                }

                cells[y as usize][x as usize] =
                    Cell::new(x as i32, y as i32, '█', fg_color, RGB::named(BLACK));
            }
        }

        self.grid = cells;
    }

    // MARK: impl {Map} -> draw
    pub fn draw(&self, ctx: &mut BTerm) {
        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.get_cell(x as i32, y as i32);
                ctx.print_color(
                    x as i32,
                    y as i32,
                    cell.fg_color(),
                    cell.bg_color(),
                    cell.char(),
                );
            }
        }
    }
}

// MARK: {State}
pub struct State {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) player: Player,
    pub(crate) map: Map,
}

// MARK: impl {State}
impl State {
    pub fn new(width: u32, height: u32, seed: u64) -> Self {
        Self {
            width,
            height,
            player: Player::new(),
            map: Map::new(width, height, seed, 225.0),
        }
    }

    pub fn draw_fps(&self, x: i32, y: i32, ctx: &mut BTerm) {
        let str = format!("FPS: {}", (1.0 / (ctx.frame_time_ms / 1000.0)).floor());
        ctx.print_color(x, y, RGB::named(WHITE), RGB::named(BLACK), str);
    }

    // MARK: impl {State} -> draw
    pub fn draw(&mut self, ctx: &mut BTerm) {
        self.map.draw(ctx);
        self.player.draw(ctx);
        self.draw_fps(0, 0, ctx);
    }
}

// MARK: impl {GameState for State} -> tick
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        let bounds = Point::new(self.width, self.height);

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::W => self.player.move_up(bounds),
                VirtualKeyCode::S => self.player.move_down(bounds),
                VirtualKeyCode::A => self.player.move_left(bounds),
                VirtualKeyCode::D => self.player.move_right(bounds),
                VirtualKeyCode::Q | VirtualKeyCode::Escape => ctx.quit(),
                VirtualKeyCode::Space => self.map.generate(0),
                _ => {}
            }
        }

        let current_cell = self.map.get_cell(self.player.x(), self.player.y());
        self.player.set_bg_color(current_cell.fg_color());

        self.draw(ctx);
    }
}
