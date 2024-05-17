use crate::entities::entity::{Drawable, EntityFuncs, EntityProps, Movable};
use crate::util::constrain_coords;
use bracket_lib::terminal::{BTerm, Point};
use bracket_lib::{
    color::{BLACK, WHITE},
    prelude::RGB,
};

// MARK: {Player}
pub struct Player {
    pub props: EntityProps,
}

// MARK: impl {Player}
impl Player {
    pub fn new() -> Self {
        Self {
            props: EntityProps::new(0, 0, 'B', RGB::named(WHITE), RGB::named(BLACK)),
        }
    }
}

// MARK: #Drawable for {Player}
impl Drawable for Player {
    fn draw(&self, ctx: &mut BTerm) {
        ctx.set(
            self.props.x,
            self.props.y,
            self.props.fg_color,
            self.props.bg_color,
            self.props.char,
        );
    }
}

// MARK: #Movable for {Player}
impl Movable for Player {
    fn move_up(&mut self, bounds: Point) {
        let new_pos = constrain_coords(self.x(), self.y() - 1, bounds.x, bounds.y);
        self.set_pos(new_pos.x, new_pos.y);
    }

    fn move_down(&mut self, bounds: Point) {
        let new_pos = constrain_coords(self.x(), self.y() + 1, bounds.x, bounds.y);
        self.set_pos(new_pos.x, new_pos.y);
    }

    fn move_left(&mut self, bounds: Point) {
        let new_pos = constrain_coords(self.x() - 1, self.y(), bounds.x, bounds.y);
        self.set_pos(new_pos.x, new_pos.y);
    }

    fn move_right(&mut self, bounds: Point) {
        let new_pos = constrain_coords(self.x() + 1, self.y(), bounds.x, bounds.y);
        self.set_pos(new_pos.x, new_pos.y);
    }
}

// MARK: #EntityFuncs for {Player}
impl EntityFuncs for Player {
    fn x(&self) -> i32 {
        self.props.x
    }

    fn y(&self) -> i32 {
        self.props.y
    }

    fn char(&self) -> char {
        self.props.char
    }

    fn fg_color(&self) -> RGB {
        self.props.fg_color
    }

    fn bg_color(&self) -> RGB {
        self.props.bg_color
    }

    fn set_x(&mut self, x: i32) {
        self.props.x = x;
    }

    fn set_y(&mut self, y: i32) {
        self.props.y = y;
    }

    fn set_char(&mut self, c: char) {
        self.props.char = c;
    }

    fn set_fg_color(&mut self, c: RGB) {
        self.props.fg_color = c;
    }

    fn set_bg_color(&mut self, c: RGB) {
        self.props.bg_color = c;
    }
}
