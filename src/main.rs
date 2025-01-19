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


    let mut treasure = String::from("Treasure");
    let friend1 = &treasure;
    let friend2 = &treasure; // can borrow immutable more than once

    let trusted_friend = &mut treasure;
    // let trusted_friend2 = &mut treasure; // cannot borrow `treasure` as mutable more than once at a time
    trusted_friend.push_str("Coins");

}
