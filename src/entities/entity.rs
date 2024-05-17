use bracket_lib::{
    color::RGB,
    terminal::{BTerm, Point},
};

// MARK: {EntityProps}
pub struct EntityProps {
    pub x: i32,
    pub y: i32,
    pub char: char,
    pub fg_color: RGB,
    pub bg_color: RGB,
}

impl EntityProps {
    pub fn new(x: i32, y: i32, c: char, fg_color: RGB, bg_color: RGB) -> Self {
        Self {
            x,
            y,
            char: c,
            fg_color,
            bg_color,
        }
    }
}

// MARK: #EntityFuncs
#[allow(unused)]
pub trait EntityFuncs {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn char(&self) -> char;
    fn fg_color(&self) -> RGB;
    fn bg_color(&self) -> RGB;
    fn set_x(&mut self, x: i32);
    fn set_y(&mut self, y: i32);
    fn set_pos(&mut self, x: i32, y: i32) {
        self.set_x(x);
        self.set_y(y);
    }
    fn set_char(&mut self, c: char);
    fn set_fg_color(&mut self, c: RGB);
    fn set_bg_color(&mut self, c: RGB);
}

// MARK: #Movable
pub trait Movable {
    fn move_up(&mut self, bounds: Point);
    fn move_down(&mut self, bounds: Point);
    fn move_left(&mut self, bounds: Point);
    fn move_right(&mut self, bounds: Point);
}

// MARK: #Drawable
pub trait Drawable {
    fn draw(&self, ctx: &mut BTerm);
}
