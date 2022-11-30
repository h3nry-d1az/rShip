use crate::console::goto_xy;

pub struct Ship {
    x: usize,
    y: usize,
    health: u8,
    lives:  u8
}

impl Ship {
    pub fn new(
        x: usize,
        y: usize,
        health: u8,
        lives: u8
    ) -> Self {
        Ship {
            x,
            y,
            health,
            lives
        }
    }

    pub fn get_x(self) -> usize { self.x }
    pub fn get_y(self) -> usize { self.y }
    pub fn get_lives(self) -> u8 { self.lives }

    pub fn hit(&mut self)   { self.health -= 1; }
    pub fn oneup(&mut self) { self.lives += 1; }

    pub fn draw(self) {
        goto_xy(self.x, self.y);
        print!("  {}", 65 as char);

        goto_xy(self.x, self.y+1);
        print!(" {}{}{}", 40 as char, 207 as char, 41 as char);

        goto_xy(self.x, self.y+2);
        print!("{}{} {}{}", 174 as char, 190 as char, 190 as char, 175 as char);
    }

    pub fn clean(self) {
        goto_xy(self.x, self.y);
        print!("      ");

        goto_xy(self.x, self.y+1);
        print!("      ");

        goto_xy(self.x, self.y+2);
        print!("      ");
    }
}