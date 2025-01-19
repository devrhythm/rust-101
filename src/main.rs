fn main() {
    let x = 5;
    let x2: i32 = 5;
    let y = 0.5;
    let y2: f64 = 0.5;

    let z = x + y as i32;

    let msg = String::from("Hello, world!");
    let msg2 = "Hello, world!".to_string();
    let msg3 = "Hello, world!";
    let msg4  = format!("{}, x:{}, y:{}, z:{}", "hello", x ,y, z);

    println!("{}", msg4);
}
