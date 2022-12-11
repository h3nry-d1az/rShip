use std::time::{Duration, Instant};
use std::thread::sleep;
use std::cmp::{ max, min };
use device_query::keymap::Keycode;
use crate::console::{
    cursor::goto_xy,
    io::getch
};
use crate::entities::asteroid::Kind;

#[derive(Debug, Clone)]
pub struct Ship {
    x: usize,
    y: usize,
    health: u8,
    lives:  u8,
    velocity: usize,
    timestamp: Instant
}

impl Ship {
    pub fn new(
        x: usize,
        y: usize,
        health: u8,
        lives: u8,
        velocity: usize
    ) -> Self {
        Ship {
            x,
            y,
            health,
            lives,
            velocity,
            timestamp: Instant::now()
        }
    }

    pub fn get_x(&self) -> usize { self.x }
    pub fn get_y(&self) -> usize { self.y }
    pub fn get_lives(&self) -> u8 { self.lives }

    pub fn hit(&mut self, asteroid: Kind) {
        match asteroid {
            Kind::Small => { self.health = max(self.health as i8 - 1, 0) as u8; },
            Kind::Medium => { self.health = max(self.health as i8 - 2, 0) as u8; },
            Kind::Huge => { self.health = 0; }
        }
    }
    pub fn oneup(&mut self) { self.lives += 1; }

    pub fn draw(&self) {
        goto_xy(self.x, self.y);
        println!("  A");

        goto_xy(self.x, self.y+1);
        println!(" (¤)");

        goto_xy(self.x, self.y+2);
        println!("«/ \\»");
    }

    pub fn clean(&self) {
        goto_xy(self.x, self.y);
        println!("      ");

        goto_xy(self.x, self.y+1);
        println!("      ");

        goto_xy(self.x, self.y+2);
        println!("      ");
    }

    pub fn tick(&mut self) {
        self.clean();
        match getch() {
            Some(Keycode::A) | Some(Keycode::Left)  => { if self.x > 3   { self.x = max(3, self.x-self.velocity);   } },
            Some(Keycode::D) | Some(Keycode::Right) => { if self.x < 111 { self.x = min(111, self.x+self.velocity); } },
            Some(Keycode::W) | Some(Keycode::Up)    => { if self.y > 4  { self.y = max(4, self.y-1);  } },
            Some(Keycode::S) | Some(Keycode::Down)  => { if self.y < 25 { self.y = min(25, self.y+1); } },
            _ => {}
        }
        self.draw();
        self.show_health();
    }

    pub fn show_health(&self) {
        goto_xy(64, 1); println!("                                                ");
        goto_xy(64, 1); println!("L I V E S : ");
        for i in 0..(self.lives) {
            goto_xy((76 + 2*i) as usize, 1); println!("X");
        }
        goto_xy(92, 1); println!("H E A L T H : {:.0}%",  self.health as f32 * 33.3);
    }

    pub fn die(&mut self) {
        if self.health <= 0 {
            self.clean();
            goto_xy(self.x, self.y);     println!("  **  ");
		    goto_xy(self.x, self.y + 1); println!(" **** ");
		    goto_xy(self.x, self.y + 2); println!("  **  ");
		    sleep(Duration::from_millis(200));

            self.clean();
            goto_xy(self.x, self.y);     println!("*  * *");
		    goto_xy(self.x, self.y + 1); println!("* ****");
		    goto_xy(self.x, self.y + 2); println!("* * **");
		    sleep(Duration::from_millis(300));

            self.clean();
            goto_xy(self.x, self.y);     println!("* ** *");
		    goto_xy(self.x, self.y + 1); println!(" **** ");
		    goto_xy(self.x, self.y + 2); println!("* ** *");
		    sleep(Duration::from_millis(200));
            self.clean();

            self.lives -= 1;
            self.health = 3;
		    self.show_health();
		    self.draw();
        }
    }

    pub fn shoot(&mut self) -> bool {
        if self.timestamp.elapsed().as_millis() > 167 {
            self.timestamp = Instant::now();
            return true;
        }

        false
    }
}