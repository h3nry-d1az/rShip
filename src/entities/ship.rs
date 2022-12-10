use std::time::Duration;
use std::thread::sleep;
use std::cmp::{ max, min };
use device_query::keymap::Keycode;
use crate::console::{
    cursor::goto_xy,
    io::{ kbhit, getch }
};

pub struct Ship {
    x: usize,
    y: usize,
    health: u8,
    lives:  u8,
    velocity: usize
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
            velocity
        }
    }

    pub fn get_x(&self) -> usize { self.x }
    pub fn get_y(&self) -> usize { self.y }
    pub fn get_lives(&self) -> u8 { self.lives }

    pub fn hit(&mut self)   { self.health -= 1; }
    pub fn oneup(&mut self) { self.lives += 1; }

    pub fn draw(&self) {
        goto_xy(self.x, self.y);
        print!("  {}", 65 as char);

        goto_xy(self.x, self.y+1);
        print!(" {}{}{}", 40 as char, 207 as char, 41 as char);

        goto_xy(self.x, self.y+2);
        print!("{}{} {}{}", 174 as char, 190 as char, 190 as char, 175 as char);
    }

    pub fn clean(&self) {
        goto_xy(self.x, self.y);
        print!("      ");

        goto_xy(self.x, self.y+1);
        print!("      ");

        goto_xy(self.x, self.y+2);
        print!("      ");
    }

    pub fn tick(&mut self) {
        if !kbhit() { return; }
        self.clean();
        match getch() {
            Some(Keycode::A) | Some(Keycode::Left)  => { if self.x > 3 { self.x = max(3, self.x-self.velocity);     } },
            Some(Keycode::D) | Some(Keycode::Right) => { if self.x < 111 { self.x = min(111, self.x+self.velocity); } },
            Some(Keycode::W) | Some(Keycode::Up)    => { if self.y > 4 { self.y = max(4, self.y-self.velocity);     } },
            Some(Keycode::S) | Some(Keycode::Down)  => { if self.y < 25 { self.y = min(25, self.y+self.velocity);   } },
            _ => return
        }
        self.draw();
        self.show_health();
    }

    pub fn show_health(&self) {
        goto_xy(70, 1); print!("Vidas: {}", self.lives);
        goto_xy(90, 1); print!("Salud: ");
        goto_xy(97, 1); print!("      ");
        for i in 0..(self.health) {
            goto_xy((97 + i) as usize, 1); print!("X");
        }
    }

    pub fn die(&mut self) {
        if self.health <= 0 {
            self.clean();
            goto_xy(self.x, self.y);     print!("  **  ");
		    goto_xy(self.x, self.y + 1); print!(" **** ");
		    goto_xy(self.x, self.y + 2); print!("  **  ");
		    sleep(Duration::from_millis(200));

            self.clean();
            goto_xy(self.x, self.y);     print!("*  * *");
		    goto_xy(self.x, self.y + 1); print!("* ****");
		    goto_xy(self.x, self.y + 2); print!("* * **");
		    sleep(Duration::from_millis(300));

            self.clean();
            goto_xy(self.x, self.y);     print!("* ** *");
		    goto_xy(self.x, self.y + 1); print!(" **** ");
		    goto_xy(self.x, self.y + 2); print!("* ** *");
		    sleep(Duration::from_millis(200));
            self.clean();

            self.lives -= 1;
            self.health = 3;
		    self.show_health();
		    self.draw();
        }
    }
}