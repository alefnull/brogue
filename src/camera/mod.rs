use bracket_lib::terminal::Point;

use crate::entities::entity::Movable;
use crate::util::constrain_coords;

pub struct Camera {
  pub cols: i32,
  pub rows: i32,
  pub origin: Point,
}

impl Camera {
  pub fn new(cols: i32, rows: i32) -> Self {
    Self {
      cols,
      rows,
      origin: Point::new(0, 0),
    }
  }
  
  pub fn set_origin(&mut self, x: i32, y: i32) {
    self.origin = Point::new(x, y);
  }
}

impl Movable for Camera {
  fn move_up(&mut self, bounds: Point) {
    let new_pos = constrain_coords(self.origin.x, self.origin.y - 1, bounds.x, bounds.y);
    self.set_origin(new_pos.x, new_pos.y);
  }

  fn move_down(&mut self, bounds: Point) {
    let new_pos = constrain_coords(self.origin.x, self.origin.y + 1, bounds.x, bounds.y);
    self.set_origin(new_pos.x, new_pos.y);
  }

  fn move_left(&mut self, bounds: Point) {
    let new_pos = constrain_coords(self.origin.x - 1, self.origin.y, bounds.x, bounds.y);
    self.set_origin(new_pos.x, new_pos.y);
  }

  fn move_right(&mut self, bounds: Point) {
    let new_pos = constrain_coords(self.origin.x + 1, self.origin.y, bounds.x, bounds.y);
    self.set_origin(new_pos.x, new_pos.y);
  }
}
