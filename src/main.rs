use std::{process::Command, error::Error};
use std::time::Duration;
use std::thread::sleep;
use rand::Rng;
use device_query::Keycode;
use rShip::{
    entities::{ship::Ship, asteroid::Asteroid, bullet::Bullet, draw_limits},
    console::{cursor::{hide_cursor, goto_xy}, io::getch}
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
             .output()?;

    draw_limits();

    goto_xy(58, 14); println!("HSHIP");
	goto_xy(41, 15); println!("Pulsa cualquier tecla para iniciar el juego");
	'pause: loop {
        match getch() {
            Some(_) => break 'pause,
            _ => continue 'pause
        }
    }
	goto_xy(58, 14); println!("     ");
	goto_xy(41, 15); println!("                                           ");

    while !game_over {
        goto_xy(7, 1); println!("Puntos: {}", score);

        if (score % 2000 == 0) && score != 0 {
			if (score % 4000 == 0) && score != 0 {
				asteroids.push(Asteroid::new(rand::thread_rng().gen_range(2..117),
                                             rand::thread_rng().gen_range(4..7)));
			}
			player.oneup();
			score += 100;
		}

        let key = getch().unwrap_or(Keycode::Tab);
        if key == Keycode::Z ||
           key == Keycode::Space ||
           key == Keycode::Dot {
            bullets.push(Bullet::new(player.get_x() + 2, player.get_y() - 1, 1));
        }
        else if key == Keycode::P {
            goto_xy(58, 14); println!("HSHIP");
            goto_xy(38, 15); println!("Pulsa cualquier tecla para continuar con el juego");
            'pause: loop {
                match getch() {
                    Some(_) => break 'pause,
                    _ => continue 'pause
                }
            }
            goto_xy(58, 14); println!("     ");
            goto_xy(38, 15); println!("                                                 ");
        }
        else if key == Keycode::L &&
                cfg!(debug_assertions) {
            score += 100;
        }
        else if key == Keycode::E &&
                cfg!(debug_assertions) {
            player.hit();
        }

        for bullet in bullets.iter_mut() {
            bullet.tick();
            if bullet.is_out() {
                goto_xy(bullet.get_x(), bullet.get_y()); println!(" ");
				drop(bullet);
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

        for (aidx, acoord) in asteroids.clone()
                                       .iter()
                                       .map(|a| (a.get_x(), a.get_y()))
                                       .enumerate() {
            for (bidx, bcoord) in bullets.clone()
                                         .iter()
                                         .map(|b| (b.get_x(), b.get_y()))
                                         .enumerate() {
                if (acoord.0 == bcoord.0) && (acoord.1 == bcoord.1 || acoord.1 + 1 == bcoord.1) {
                    goto_xy(bcoord.0, bcoord.1); println!(" ");
                    bullets.remove(bidx);

                    goto_xy(acoord.0, acoord.1); println!(" ");
                    asteroids.remove(aidx);
                    asteroids.push(Asteroid::new(rand::thread_rng().gen_range(2..117),
                                                 rand::thread_rng().gen_range(4..7)));

                    score += 100;
                }
            }
        }

        if player.get_lives() <= 0 {
            game_over = true;
        }

        sleep(Duration::from_millis(30));
    }

    while game_over {
        goto_xy(55, 14); println!("GAME OVER");
		goto_xy(43, 15); println!("Pulsa ESPACIO para salir del juego");

        match getch() {
            Some(Keycode::Space) => break,
            _ => continue
        }
    }

    Ok(())
}
