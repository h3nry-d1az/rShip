use std::{process::Command, error::Error};
use std::time::Duration;
use std::thread::sleep;
use device_query::Keycode;
use rShip::entities::asteroid::Kind;
use rShip::{
    entities::{ship::Ship, asteroid::Asteroid, bullet::Bullet, draw_limits},
    console::{cursor::{hide_cursor, goto_xy}, io::getch}
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut game_over = false;
    let mut score: usize = 0;

    let mut player = Ship::new(57, 25, 3, 4, 2);
    let mut asteroids: Vec<Asteroid> = Vec::new();
    let mut bullets: Vec<Bullet> = Vec::new();

    for _ in 0..6 {
        asteroids.push(Asteroid::spawn());
    }

    hide_cursor();
    Command::new("cmd")
             .args(["/C", "title hShip, a small ship game made of ASCII characters"])
             .output()?;

    draw_limits();

    goto_xy(30, 11); println!("      :::    :::  ::::::::  :::    ::: ::::::::::: ::::::::: ");
    goto_xy(30, 12); println!("     :+:    :+: :+:    :+: :+:    :+:     :+:     :+:    :+: ");
    goto_xy(30, 13); println!("    +:+    +:+ +:+        +:+    +:+     +:+     +:+    +:+  ");
    goto_xy(30, 14); println!("   +#++:++#++ +#++:++#++ +#++:++#++     +#+     +#++:++#+    ");
    goto_xy(30, 15); println!("  +#+    +#+        +#+ +#+    +#+     +#+     +#+           ");
    goto_xy(30, 16); println!(" #+#    #+# #+#    #+# #+#    #+#     #+#     #+#            ");
    goto_xy(30, 17); println!("###    ###  ########  ###    ### ########### ###             ");


	goto_xy(27, 20); println!("P R E S S   A N Y   K E Y   T O   S T A R T   T H E   G A M E");
	'pause: loop {
        match getch() {
            Some(_) => break 'pause,
            _ => continue 'pause
        }
    }
    goto_xy(30, 11); println!("                                                             ");
    goto_xy(30, 12); println!("                                                             ");
    goto_xy(30, 13); println!("                                                             ");
    goto_xy(30, 14); println!("                                                             ");
    goto_xy(30, 15); println!("                                                             ");
    goto_xy(30, 16); println!("                                                             ");
    goto_xy(30, 17); println!("                                                             ");


	goto_xy(27, 20); println!("                                                             ");

    while !game_over {
        goto_xy(7, 1); println!("S C O R E : {}", score);

        if (score % 2000 == 0) && score != 0 {
			if (score % 4000 == 0) && score != 0 {
				asteroids.push(Asteroid::spawn());
			}
			player.oneup();
			score += 100;
		}

        let key = getch().unwrap_or(Keycode::Tab);
        if (key == Keycode::Z ||
            key == Keycode::Space ||
            key == Keycode::Dot) &&
           player.shoot() {
            bullets.push(Bullet::new(player.get_x() + 2, player.get_y() - 1, 1));
        }
        else if key == Keycode::P {
            goto_xy(30, 11); println!("      :::    :::  ::::::::  :::    ::: ::::::::::: ::::::::: ");
            goto_xy(30, 12); println!("     :+:    :+: :+:    :+: :+:    :+:     :+:     :+:    :+: ");
            goto_xy(30, 13); println!("    +:+    +:+ +:+        +:+    +:+     +:+     +:+    +:+  ");
            goto_xy(30, 14); println!("   +#++:++#++ +#++:++#++ +#++:++#++     +#+     +#++:++#+    ");
            goto_xy(30, 15); println!("  +#+    +#+        +#+ +#+    +#+     +#+     +#+           ");
            goto_xy(30, 16); println!(" #+#    #+# #+#    #+# #+#    #+#     #+#     #+#            ");
            goto_xy(30, 17); println!("###    ###  ########  ###    ### ########### ###             ");
            goto_xy(24, 20); println!("P R E S S   A N Y   K E Y   T O   C O N T I N U E   T H E   G A M E");
            'pause: loop {
                sleep(Duration::from_millis(100));
                match getch() {
                    Some(_) => break 'pause,
                    _ => continue 'pause
                }
            }
            goto_xy(30, 11); println!("                                                             ");
            goto_xy(30, 12); println!("                                                             ");
            goto_xy(30, 13); println!("                                                             ");
            goto_xy(30, 14); println!("                                                             ");
            goto_xy(30, 15); println!("                                                             ");
            goto_xy(30, 16); println!("                                                             ");
            goto_xy(30, 17); println!("                                                             ");
            goto_xy(24, 20); println!("                                                                   ");
        }
        else if key == Keycode::L &&
                cfg!(debug_assertions) {
            score += 100;
        }
        else if key == Keycode::E &&
                cfg!(debug_assertions) {
            player.hit(Kind::Small);
        }

        for bullet in bullets.clone()
                             .iter()
                             .enumerate()
                             .map(|d| (d.1.get_x(), d.1.get_y(), d.1.is_out(), d.0)) {
            match bullets.get_mut(bullet.3) {
                Some(b) => b.tick(),
                None => continue
            }
            if bullet.2 {
                goto_xy(bullet.0, bullet.1 - 1); println!(" ");
                bullets.remove(bullet.3);
            }
        }

        player.tick();
        player.show_health();
        player.die();

        for (idx, _) in asteroids.clone()
                                 .iter_mut()
                                 .enumerate() {
            match asteroids.get(idx) {
                Some(_) => {},
                None => continue 
            }
            asteroids[idx].tick();
            if asteroids[idx].collides(&player) {
                asteroids[idx].clean();
                player.hit(asteroids[idx].get_kind());
                player.clean();
                player.draw();
                player.show_health();
		        asteroids.remove(idx);
                asteroids.push(Asteroid::spawn());
            }
        }

        for (aidx, asteroid) in asteroids.clone()
                                       .iter()
                                       .enumerate() {
            for (bidx, bullet) in bullets.clone()
                                         .iter()
                                         .enumerate() {
                if asteroid.gets_hit(&bullet) {
                    goto_xy(bullet.get_x(), bullet.get_y()); println!(" ");
                    if bidx < bullets.len() {
                        bullets.remove(bidx);
                    }

                    asteroids[aidx].hit();
                    if asteroids[aidx].is_dead() {
                        match asteroids[aidx].get_kind() {
                            Kind::Small => { score += 100; },
                            Kind::Medium => { score += 300; },
                            Kind::Huge => { score += 500; }
                        }
                        asteroids[aidx].clean();
                        asteroids.remove(aidx);
                        asteroids.push(Asteroid::spawn());
                    }
                }
            }
        }

        if player.get_lives() <= 0 {
            game_over = true;
        }

        sleep(Duration::from_millis(30));
    }

    while game_over {
        goto_xy(18, 11); println!("  ________    _____      _____  ___________ ____________   _________________________ ");
        goto_xy(18, 12); println!(" /  _____/   /  _  \\    /     \\ \\_   _____/ \\_____  \\   \\ /   /\\_   _____/\\______   \\");
        goto_xy(18, 13); println!("/   \\  ___  /  /_\\  \\  /  \\ /  \\ |    __)_   /   |   \\   Y   /  |    __)_  |       _/");
        goto_xy(18, 14); println!("\\    \\_\\  \\/    |    \\/    Y    \\|        \\ /    |    \\     /   |        \\ |    |   \\");
        goto_xy(18, 15); println!(" \\______  /\\____|__  /\\____|__  /_______  / \\_______  /\\___/   /_______  / |____|_  /");
        goto_xy(18, 16); println!("        \\/         \\/         \\/        \\/          \\/                 \\/         \\/ ");
        goto_xy(29, 19); println!("P R E S S   [S P A C E]   T O   Q U I T   T H E   G A M E");

        match getch() {
            Some(Keycode::Space) => break,
            _ => continue
        }
    }

    Ok(())
}
