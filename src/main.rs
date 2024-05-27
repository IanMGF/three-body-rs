mod body;

use body::vectors::_3DVector;
use body::Body;
use std::time;

fn main() {
    let mut body_1 = Body {
        pos: _3DVector {
            x: 100f64,
            y: -100f64,
            z: -100f64,
        },
        mass: 1e8f64,
        velocity: Default::default(),
    };

    let mut body_2 = Body {
        pos: _3DVector {
            x: -100f64,
            y: 100f64,
            z: 100f64,
        },
        mass: 3e8f64,
        velocity: Default::default(),
    };

    let mut body_3 = Body {
        pos: _3DVector {
            x: -10f64,
            y: -100f64,
            z: 100f64,
        },
        mass: 1e9f64,
        velocity: Default::default(),
    };

    let start: time::Instant = time::Instant::now();
    let mut total_time_secs = 0f64;
    for step in 0i64..(1e11 as i64) {
        let new_time = start.elapsed().as_secs_f64();
        let dt_time = (new_time - total_time_secs) * 10f64;
        total_time_secs = new_time;

        if step % (4e5 as i64) == 0 {
            print!("\x1B[2J\x1B[1;1H");
            println!(
                "Time: {:<7} ",
                format!("{:.2}s", start.elapsed().as_secs_f64())
            );
            println!("B1: {:?}", body_1);
            println!("B2: {:?}", body_2);
            println!("B3: {:?}", body_3);
            println!(
                "C:  {:?}",
                (body_1.pos * body_1.mass + body_2.pos * body_2.mass + body_3.pos * body_3.mass)
                    / 3f64
            );
        }

        let g12 = body_1 << body_2;
        let g13 = body_1 << body_3;
        let g23 = body_2 << body_3;

        body_1.velocity += dt_time * (g12 + g13) / body_1.mass;
        body_2.velocity += dt_time * (-g12 + g23) / body_2.mass;
        body_3.velocity += dt_time * (-g13 - g23) / body_3.mass;

        body_1.pos += dt_time * body_1.velocity;
        body_2.pos += dt_time * body_2.velocity;
        body_3.pos += dt_time * body_3.velocity;
    }
}
