use std::string;

fn main() {
    let x = 5;
    let x2: i32 = 5;
    let y = 0.5;
    let y2: f64 = 0.5;

    let z = x + y as i32;

    let msg = String::from("Hello, world!");
    let msg2 = "Hello, world!".to_string();
    let msg3 = "Hello, world!";
    let msg4 = format!("{}, x:{}, y:{}, z:{}", "hello", x, y, z);

    println!("{}", msg4);

    let mut treasure = String::from("Treasure");
    let friend1 = &treasure;
    let friend2 = &treasure; // can borrow immutable more than once

    let trusted_friend = &mut treasure;
    // let trusted_friend2 = &mut treasure; // cannot borrow `treasure` as mutable more than once at a time
    trusted_friend.push_str("Coins");

    // sample lifetime
    let treasure2;
    {
        let local_treasure = String::from("Treasure2");
        treasure2 = &local_treasure; // borrowed value does not live long enough
    }
    // println!("{}", treasure2);  // treasure 2 no longer valid because local_treasure is dropped

    let map1 = "Ancient map of the sea";
    let map2 = "Map to hidden gold";

    let chosen_map = longest_map(map1, map2);
    println!("Crabby's longest map: {}", chosen_map);

    // sample struct
    let mut crabby = Crabby {
        name: "Crabby".to_string(),
        health: 100,
    };
    crabby.take_damage(100);
    crabby.take_damage(10);
    println!("Crabby: {}, health: {} ", crabby.name, crabby.health);

    crabby.heal(60);
    println!("Crabby: {}, health: {}", crabby.name, crabby.health);
}

// Lifetimes in Rust ensure that references are valid for as long as needed.
// They prevent dangling references and memory safety issues by specifying
// how long references should be valid. Lifetimes are denoted using an
// apostrophe followed by a name ('<lifetime-name>),  like 'a.
fn longest_map<'a>(map1: &'a str, map2: &'a str) -> &'a str {
    if map1.len() > map2.len() {
        map1
    } else {
        map2
    }
}

struct Crabby {
    name: String,
    health: u8,
}

impl Crabby {
    fn take_damage(&mut self, damage: u8) {
        self.health = self.health.saturating_sub(damage);
    }

    fn heal(&mut self, health: u8) {
        if self.health + health >= 100 {
            self.health = 100;
        } else {
            self.health += health;
        }
    }
}
