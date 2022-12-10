use crate::console::cursor::goto_xy;

#[derive(Debug, Clone)]
pub struct Bullet {
    x: usize,
    y: usize,
    velocity: usize
}

impl Bullet {
    pub fn new(x: usize, y: usize, velocity: usize) -> Bullet {
        Bullet { x, y, velocity }
    }

    pub fn get_x(&self) -> usize { self.x }
    pub fn get_y(&self) -> usize { self.y }

    pub fn tick(&mut self) {
        goto_xy(self.x, self.y); println!(" ");
	    self.y -= self.velocity;
        goto_xy(self.x, self.y); println!("{}", 186 as char);
    }

    pub fn is_out(&self) -> bool {
        self.y == 4
    }
}