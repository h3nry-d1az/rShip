use std::time::Duration;
use std::thread::sleep;
use rand::Rng;
use crate::console::cursor::goto_xy;
use crate::entities::ship::Ship;
use crate::entities::bullet::Bullet;

#[derive(Debug, Clone)]
pub enum Kind {
    Small,
    Medium,
    Huge
}

#[derive(Debug, Clone)]
pub struct Asteroid {
    kind: Kind,
    health: usize,
    x: usize,
    y: usize,
}

impl Asteroid {
    pub fn new(kind: Kind, x: usize, y: usize) -> Asteroid {
        match kind {
            Kind::Small =>  Asteroid { kind, health: 1, x, y },
            Kind::Medium => Asteroid { kind, health: 3, x, y },
            Kind::Huge =>   Asteroid { kind, health: 5, x, y }
        }
    }

    pub fn spawn() -> Asteroid {
        Asteroid::new(match rand::thread_rng().gen_range(0..10) {
            0..=2 => Kind::Huge,
            3..=7 => Kind::Medium,
            8..=9 => Kind::Small,
            _ => Kind::Medium
        },
            rand::thread_rng().gen_range(6..110),
            rand::thread_rng().gen_range(4..9))
    }

    pub fn get_x(&self) -> usize { self.x }
    pub fn get_y(&self) -> usize { self.y }
    pub fn get_kind(&self) -> Kind { self.kind.clone() }

    pub fn set_x(&mut self, val: usize) { self.x = val }
    pub fn set_y(&mut self, val: usize) { self.y = val }

    pub fn draw(&self) {
        match self.kind {
            Kind::Small => {
                goto_xy(self.x, self.y);
                println!("▓");
            },
            Kind::Medium => {
                goto_xy(self.x, self.y);
                println!("▒▒");
                goto_xy(self.x, self.y+1);
                println!("▒▒");
            },
            Kind::Huge => {
                goto_xy(self.x, self.y);
                println!("░░░");
                goto_xy(self.x, self.y+1);
                println!("░░░");
            }
        }
    }

    pub fn clean(&self) {
        match self.kind {
            Kind::Small => {
                goto_xy(self.x, self.y);
                println!(" ");
            },
            Kind::Medium => {
                goto_xy(self.x, self.y);
                println!("  ");
                goto_xy(self.x, self.y+1);
                println!("  ");
            },
            Kind::Huge => {
                goto_xy(self.x, self.y);
                println!("   ");
                goto_xy(self.x, self.y+1);
                println!("   ");
            }
        }
    }

    pub fn tick(&mut self) {
        self.clean();
        self.y += 1;

	    if self.y > 26 {
		    (self.kind, self.health) = match rand::thread_rng()
                                                   .gen_range(0..10) {
                0 => (Kind::Huge, 5),
                1..=2 => (Kind::Medium, 3),
                3..=9 => (Kind::Small, 1),
                _ => (Kind::Small, 1)
            };
            (self.x, self.y) = (rand::thread_rng().gen_range(6..110),
                                rand::thread_rng().gen_range(4..7));
        }

        sleep(Duration::from_millis(5));
	    self.draw();
    }

    pub fn collides(&self, ship: &Ship) -> bool {
        match self.kind {
            Kind::Small => (self.x >= ship.get_x() && self.x < ship.get_x() + 5) &&
                           (self.y >= ship.get_y() && self.y < ship.get_y() + 3),
            Kind::Medium => (self.x >= ship.get_x()-1 && self.x < ship.get_x() + 5) &&
                            (self.y >= ship.get_y()-1 && self.y < ship.get_y() + 3),
            Kind::Huge => (self.x >= ship.get_x()-2 && self.x < ship.get_x() + 5) &&
                          (self.y >= ship.get_y()-1 && self.y < ship.get_y() + 3)
        }
    }

    pub fn gets_hit(&self, bullet: &Bullet) -> bool {
        match self.kind {
            Kind::Small => (self.x == bullet.get_x()) &&
                           (self.y == bullet.get_y() || self.y + 1 == bullet.get_y()),
            Kind::Medium => (self.x == bullet.get_x() || self.x + 1 == bullet.get_x()) &&
                            (self.y == bullet.get_y() || self.y + 1 == bullet.get_y() || self.y + 2 == bullet.get_y()),
            Kind::Huge => (self.x == bullet.get_x() || self.x + 1 == bullet.get_x() || self.x + 2 == bullet.get_x()) &&
                          (self.y == bullet.get_y() || self.y + 1 == bullet.get_y() || self.y + 2 == bullet.get_y()),
        }
    }

    pub fn hit(&mut self) {
        self.health -= 1;
    }

    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }
}