pub mod ship;
pub mod bullet;
pub mod asteroid;

use crate::console::cursor::goto_xy;
pub fn draw_limits() {
    goto_xy(2, 3);    print!("{}", 201 as char);
	goto_xy(117, 3);  print!("{}", 187 as char);
	goto_xy(2, 28);   print!("{}", 200 as char);
	goto_xy(117, 28); print!("{}", 188 as char);
	for i in 3..117 {
		goto_xy(i, 3);  print!("{}", 205 as char);
		goto_xy(i, 28); print!("{}", 205 as char);
	}
	for i in 4..28 {
		goto_xy(2, i);   print!("{}", 186 as char);
		goto_xy(117, i); print!("{}", 186 as char);
	}
}