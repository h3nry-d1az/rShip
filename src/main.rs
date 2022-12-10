use std::{process::Command, error::Error};
use rand::Rng;
use device_query::Keycode;
use rShip::{
    entities::{ship::Ship, asteroid::Asteroid, bullet::Bullet, draw_limits},
    console::{cursor::{hide_cursor, goto_xy}, io::{kbhit, getch}}
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut game_over = false;
    let mut score: usize = 0;

    let mut player = Ship::new(57, 25, 3, 4, 1);
    let mut asteroids: Vec<Asteroid> = Vec::new();
    let mut bullets: Vec<Bullet> = Vec::new();

    for _ in 0..5 {
        asteroids.push(Asteroid::new(rand::thread_rng().gen_range(2..117),
                                     rand::thread_rng().gen_range(4..7)));
    }

    hide_cursor();
    Command::new("cmd")
             .args(["/C", "title hShip, un juego de naves en caracteres ASCII"])
             .output()
             .expect("Falló en cambiar el título de la ventana");

    draw_limits();

    goto_xy(58, 14); print!("HSHIP");
	goto_xy(41, 15); print!("Pulsa cualquier tecla para iniciar el juego");
	Command::new("cmd")
             .args(["/C", "pause > nul"])
             .output()
             .expect("Falló en pausar el juego");
	goto_xy(58, 14); print!("     ");
	goto_xy(41, 15); print!("                                           ");

    while !game_over {
        goto_xy(7, 1); print!("Puntos: {}", score);

        if (score % 2000 == 0) && score != 0 {
			if (score % 4000 == 0) && score != 0 {
				asteroids.push(Asteroid::new(rand::thread_rng().gen_range(2..117),
                                             rand::thread_rng().gen_range(4..7)));
			}
			player.oneup();
			score += 100;
		}

        if kbhit() {
            let key = getch().unwrap_or(Keycode::Tab);
            if key == Keycode::Z ||
               key == Keycode::Space ||
               key == Keycode::Dot {
                bullets.push(Bullet::new(player.get_x() + 2, player.get_y() - 1, 1));
            }
            else if key == Keycode::P {
                goto_xy(58, 14); print!("HSHIP");
                goto_xy(38, 15); print!("Pulsa cualquier tecla para continuar con el juego");
                Command::new("cmd")
                         .args(["/C", "pause > nul"])
                         .output()
                         .expect("Falló en pausar el juego");
                goto_xy(58, 14); print!("     ");
                goto_xy(38, 15); print!("                                                 ");
            }
            else if key == Keycode::L &&
                    cfg!(debug_assertions) {
                score += 100;
            }
            else if key == Keycode::E &&
                    cfg!(debug_assertions) {
                player.hit();
            }
        }

        for bullet in bullets.iter_mut() {
            bullet.tick();
            if bullet.is_out() {
                goto_xy(bullet.get_x(), bullet.get_y()); print!(" ");
				drop(bullet)
            }
        }

        player.tick();
        player.show_health();
        player.die();

        for asteroid in asteroids.iter_mut() {
            asteroid.tick();
            if asteroid.collides(&player) {
                player.hit();
                player.clean();
                player.draw();
                player.show_health();
		        asteroid.set_x(rand::thread_rng().gen_range(3..116));
                asteroid.set_y(4);
            }
        }

        todo!("terminar de implementar");
    }

    Ok(())
}
