pub mod ship;
pub mod bullet;
pub mod asteroid;

use crate::console::cursor::goto_xy;
pub fn draw_limits() {
    goto_xy(2, 3);    println!("╔");
	goto_xy(117, 3);  println!("╗");
	goto_xy(2, 28);   println!("╚");
	goto_xy(117, 28); println!("╝");
	for i in 3..117 {
		goto_xy(i, 3);  println!("═");
		goto_xy(i, 28); println!("═");
	}
	for i in 4..28 {
		goto_xy(2, i);   println!("║");
		goto_xy(117, i); println!("║");
	}
}