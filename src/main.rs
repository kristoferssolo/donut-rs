fn main() {
    const HEIGHT: i32 = 24;
    const WIDTH: i32 = 80;
    const SCREEN_SIZE: usize = (HEIGHT * WIDTH) as usize;
    let mut a: f32 = 0.0;
    let mut b: f32 = 0.0;

    print!("\x1B[2J"); // clears screen

    loop {
        let mut z: [f32; SCREEN_SIZE] = [0.0; SCREEN_SIZE];
        let mut screen: [char; SCREEN_SIZE] = [' '; SCREEN_SIZE];

        for j in (0..=628).step_by(7) {
            let j: f32 = 0.01 * j as f32;

            for i in (0..=628).step_by(2) {
                let i: f32 = 0.01 * i as f32;

                let sin_a: f32 = a.sin();
                let cos_a: f32 = a.cos();
                let sin_b: f32 = b.sin();
                let cos_b: f32 = b.cos();

                let sin_i: f32 = i.sin();
                let cos_i: f32 = i.cos();
                let sin_j: f32 = j.sin();
                let cos_j: f32 = j.cos();

                let cos_j2: f32 = cos_j + 2.0;
                let mess: f32 = 1.0 / (sin_i * cos_j2 * sin_a + sin_j * cos_a + 5.0);
                let t: f32 = sin_i * cos_j2 * cos_a - sin_j * sin_a;
                // 40 is the left screen shift
                let x: i32 = (40.0 + 30.0 * mess * (cos_i * cos_j2 * cos_b - t * sin_b)) as i32;
                // 12 is the bottom screen shift
                let y: i32 = (12.0 + 15.0 * mess * (cos_i * cos_j2 * sin_b + t * cos_b)) as i32;
                let o: usize = (x + WIDTH * y) as usize;
                // multiplying by 8 to bring in range 0-11 as 8*(sqrt(2))=11
                // because we have 11 luminance characters
                let n: i32 = (8.0
                    * ((sin_j * sin_a - sin_i * cos_j * cos_a) * cos_b
                        - sin_i * cos_j * sin_a
                        - sin_j * cos_a
                        - cos_i * cos_j * sin_b)) as i32;

                // debug
                // println!("{x} {y} {o} {n}");

                // if x,y inside screen and previous z-buffer is < mess
                // i.e. when z[o] is 0 or the prev point is behind the new point
                // so we change it to the point nearer to the eye/ above prev point

                if 0 < y && y < HEIGHT && 0 < x && x < WIDTH && mess > z[o] {
                    z[o] = mess;
                    if n > 0 {
                        let m: usize = n as usize;
                        screen[o] = ".,-~:;=!*#$Q".chars().nth(m).unwrap();
                    } else {
                        screen[o] = '.';
                    }
                }
            }
        }
        // print
        print!("\x1B[2J");
        for index in 0..screen.len() {
            let i: i32 = index as i32;
            if i % WIDTH == 0 {
                println!();
            } else {
                print!("{}", screen[index]);
            }
        }

        // increments
        a += 0.004;
        b += 0.002;
    }
}
