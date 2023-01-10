use garden::vegetables;

pub mod garden;
fn main() {
    let one = vegetables::Tomato {};
    println!("{:?}", one);
    println!("{:?}", one);
    println!("{:?}", one);

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}
