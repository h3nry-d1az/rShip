use std::time::Duration;
use std::thread::sleep;
use rand::Rng;
use crate::console::cursor::goto_xy;
use crate::entities::ship::Ship;

#[derive(Debug, Clone)]
pub struct Asteroid {
    x: usize,
    y: usize,
}

impl Asteroid {
    pub fn new(x: usize, y: usize) -> Asteroid {
        Asteroid { x, y }
    }

    pub fn get_x(&self) -> usize { self.x }
    pub fn get_y(&self) -> usize { self.y }

    pub fn set_x(&mut self, val: usize) { self.x = val }
    pub fn set_y(&mut self, val: usize) { self.y = val }

    pub fn draw(&self) {
        goto_xy(self.x, self.y);
        println!("▓");
    }

    pub fn clean(&self) {
        goto_xy(self.x, self.y);
        println!(" ");
    }

    pub fn tick(&mut self) {
        self.clean();
        self.y += 1;

	    if self.y > 27 {
		    self.x = rand::thread_rng().gen_range(5..116);
		    self.y = 4;
        }

        sleep(Duration::from_millis(5));
	    self.draw();
    }

    pub fn collides(&self, ship: &Ship) -> bool {
        (self.x >= ship.get_x() && self.x < ship.get_x() + 5) &&
        (self.y >= ship.get_y() && self.y < ship.get_y() + 3)
    }
}