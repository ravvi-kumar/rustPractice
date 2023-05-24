fn main() {
    println!("Hello, world!");
    let (mut x, mut y) = (1, 2);
    x = 3;
    println!("x:{} y:{}", x, y);
    y = 7;
    println!("y:{}", y);
    let z = 4i32;
    println!("{z}", z = z);
    let y = y;
    // y=6
    // cannot mutate immutable variable `y`
    println!("{0}", y);
    // ---------------------------------Functions----------------------------------- //
    let sum = add_numbers(12.6, 45);
    println!("sum :{}", sum)
    // -------------------------------------------------------------------- --------//
}

fn add_numbers(a: f32, b: i32) -> f32 {
    a + b as f32
}
