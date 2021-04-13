use num::complex::Complex;
use serde::Serialize;
use std;

#[derive(Serialize)]
struct SomeStruct {
    some_int: isize,
    some_str: String,
}

fn main() {
    println!("Hallo, Welt!");

    let s = SomeStruct {
        some_int: 42,
        some_str: "Hallo, Serde!".to_string(),
    };

    println!("{}", serde_json::to_string(&s).unwrap());

    println!("Return zum Fortfahren drücken...");

    {
        let mut _line = String::new();
        std::io::stdin().read_line(&mut _line).unwrap();
    }

    let mut args = std::env::args();
    args.next(); // argv[0] überspringen

    // týndur gibt \n immer als neue Zeile aus, auch am Ende der Zeile.
    // Um Leerzeilen zu vermeiden, müssen wir uns hier also auf 79
    // Reihen beschränken.
    let w = 79;
    let h = 24;

    let n_str = args.next().unwrap_or("42".to_string());
    let n =
        if let Ok(x) = n_str.parse() {
            x
        } else {
            eprintln!("{} ist keine gültige Ganzzahl", n_str);
            std::process::exit(1);
        };

    for yi in 0..h {
        let y = 1.0 - (yi as f32) / (h as f32) * 2.0;

        for xi in 0..w {
            let x = (xi as f32) / (w as f32) * 2.5 - 1.5;

            let c = Complex::new(x, y);
            let mut z = Complex::new(0.0, 0.0);
            let mut bounded = true;
            for _ in 0..n {
                z = z * z + c;
                bounded = z.norm() <= 2.0;
                if !bounded {
                    break;
                }
            }

            if bounded {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}
