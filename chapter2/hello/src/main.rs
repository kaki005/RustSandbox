mod mandelbrot;

fn is_even(n : i32) ->bool {
    n % 2 == 0
}


fn main() {
    println!("Hello, world!");

    let twenty = 20;
    let twentyone :i32 = 21;
    let twenty_two = 22i32;
    let add = twenty + twentyone;
    println!("{} + {} = {}", twenty, twentyone, add);
    let forty = [40.0, 40f32, 40.0_f32];

    println!("base 10 : {}", twenty);
    println!("base 2 : {:b}", twenty);
    println!("base 8 : {:o}", twenty);
    println!("base 16 : {:x}", twenty);

    let num = 4;
    let description = if is_even(num) {
        "偶数"
    }
    else {
        "奇数"
    };
    println!("{}は{}です。", num, description);

    let hatstack = [1, 1, 2, 5, 14,20, 40, 80, 42, 132, 429, 1430];


    for item in &hatstack {
        match item {
            1 => {println!("{} は1", item);},
            10 ..= 20 => {println!("{} は10より大きい20以下", item);},        // 上限を含む範囲
            40 | 80 => {println!("{} は40か80", item);},          // どちらか
            _ => {println!("{} はそれ以外", item);},
        }
    }

    let mandelbrot = mandelbrot::calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 80, 24);
    mandelbrot::render_mandelbrot(mandelbrot);
}
