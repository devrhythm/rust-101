use std::fmt;
use std::string;
use std::vec;
use std::collections::HashMap;

fn main() {
    sample_types();
    sample_string();
    sample_lifetime();
    sample_struct_and_enum();
    sample_threat_and_generic();
    sample_loop();
    sample_vector();
    sample_iterators_and_closures();
    sample_hashmaps();
}

fn sample_types() {
    let x = 5; // inferred as i32
    let x2: i32 = 5;
    let y = 0.5; // inferred as f64
    let y2: f64 = 0.5;

    let z = x + y as i32;

    let msg = String::from("Hello, world!"); // allocating memory on heap, use when need to modify string
    let msg2 = "Hello, world!".to_string();
    let msg3 = "Hello, world!"; // borrowed string, immutable, read-only
    let msg4 = format!(
        "{}, x:{}, x2:{}, y:{}, y2:{}, z:{}",
        "hello", x, x2, y, y2, z
    );

    println!("{} {} {} {}", msg, msg2, msg3, msg4);
}

fn sample_string() {
    let mut treasure = String::from("Treasure");
    let friend1 = &treasure;
    let friend2 = &treasure; // can borrow immutable more than once
    println!("friends: {} {}", friend1, friend2);

    let trusted_friend = &mut treasure;
    // let trusted_friend2 = &mut treasure; // cannot borrow `treasure` as mutable more than once at a time
    trusted_friend.push_str("Coins");
    println!("trusted friend: {}", trusted_friend);

    let map: String = String::from("Old map");
    let borrowed_map: &str = map.as_str();
    let mut crabby_map: String = borrowed_map.to_string();
    crabby_map.push_str(" to hidden treasure");

    println!("Crabby's map: {}", crabby_map);

}

fn sample_lifetime() {
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
}

fn sample_struct_and_enum() {
    // sample struct & enum
    let mut crabby = Crabby {
        name: "Crabby".to_string(),
        health: 100,
        state: CrabbyState::Resting,
    };
    crabby.take_damage(100);
    crabby.take_damage(10);
    println!(
        "Crabby: {}, health: {}, state: {} ",
        crabby.name, crabby.health, crabby.state
    );

    crabby.state_represent();
    crabby.heal(60);
    println!(
        "Crabby: {}, health: {}, state: {}",
        crabby.name, crabby.health, crabby.state
    );
    crabby.state_represent();

    crabby.collecting(15);
    println!(
        "Crabby: {}, health: {}, state: {}",
        crabby.name, crabby.health, crabby.state
    );
    crabby.state_represent();
}

fn sample_threat_and_generic() {
    // sample trait & generic
    let gold = Inventory { item: 100 };
    gold.display();

    let armor = Inventory {
        item: "Iron Armor".to_string(),
    };
    armor.display();
}

fn sample_loop() {
    // Sample Loop
    let mut gold_collected = 0;
    loop {
        gold_collected += 1;
        println!("Gold collected: {}", gold_collected);

        if gold_collected >= 5 {
            break;
        }
    }

    let mut energy = 10;

    while energy > 0 {
        println!("Energy: {}", energy);
        energy -= 1;
    }
    println!("Out of energy");

    let items = ["Sword", "Shield", "Potion", "Gold"];

    for item in items.iter() {
        if item == &"Potion" {
            continue;
        }

        println!("Item: {}", item);
    }

    for number in (1..4).rev() {
        println!("Number: {}", number);
    }

    for number in 1..=5 {
        println!("Number: {}", number);
    }
}

fn sample_vector() {
    // Sample vector
    let mut skills: Vec<String> = Vec::new();

    skills.push("Stun".to_string());
    skills.push("Heal".to_string());

    // .iter() returns an iterator over the vector, borrowing each element
    // .into_iter() returns an iterator that takes ownership of the vector
    // .iter_mut() returns an iterator that allows modifying each element
    // .drain() returns an iterator that removes and yields each element
    // .remove() removes and returns the element at the specified index
    for skill in skills.iter() {
        println!("Skill: {}", skill);
    }

    let mut weapons: Vec<&str> = vec!["Sword", "Shield", "Staff", "Bow"];
    let last_weapon = weapons.pop();
    let second_weapon = weapons.remove(1);
    let weapons_count = weapons.len();
    let weapons_capacity = weapons.capacity();

    println!(
        "last_weapon: {:?}, second_weapon: {:?}, weapons_count: {}, weapons_capacity: {}",
        last_weapon, second_weapon, weapons_count, weapons_capacity
    );
    // last_weapon: Some("Bow"), second_weapon: "Shield", weapons_count: 2, weapons_capacity: 4

    println!("weapons: {:?}", weapons);
    // weapons: ["Sword", "Staff"]

    weapons.push("Dagger");
    weapons.push("Axe");
    weapons.push("Knuckles");
    println!(
        "weapons_count: {}, weapons_capacity: {}",
        weapons.len(),
        weapons.capacity()
    );
    // weapons_count: 5, weapons_capacity: 8 // capacity doubled when reached
}

fn sample_iterators_and_closures() {
    let quests = vec!["Find the treasure", "Defeat the dragon", "Save the princess"];

    // closure is inline function
    let completed_quests: Vec<String> = quests
        .iter()
        .map(|quest| format!("Completed: {}", quest))
        .collect();

    println!("completed_quests: {:?}", completed_quests);

    let add = |a,b| a + b;
    let result = add(1, 2);
    println!("result: {}", result);
}

fn sample_hashmaps() {
    let mut inventory: HashMap<&str, i32> = HashMap::new();

    inventory.insert("Gold", 100);
    inventory.insert("Sword", 1);
    inventory.insert("Shield", 1);

    let gold = inventory.get("Gold");
    let sword = inventory.get("Sword");

    println!("gold: {:?}, sword: {:?}", gold, sword);

    if let Some(gold) = inventory.get_mut("Gold") {
        // Dereference operator (*) is used to access the value that the reference points to
        // Using dereference operator to modify the value in the HashMap
        *gold += 10;
    }

   println!("inventory: {:?}", inventory);
}

// ** function, structm enum, trait, and impl **

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
    state: CrabbyState,
}

impl Crabby {
    fn take_damage(&mut self, damage: u8) {
        self.health = self.health.saturating_sub(damage);
        self.state = CrabbyState::Fighting;
    }

    fn heal(&mut self, health: u8) {
        if self.health + health >= 100 {
            self.health = 100;
        } else {
            self.health += health;
        }
        self.state = CrabbyState::Resting;
    }

    fn collecting(&mut self, amount: u32) {
        self.state = CrabbyState::Collecting(amount);
    }

    fn state_represent(&self) {
        match self.state {
            CrabbyState::Resting => println!("Crabby is resting"),
            CrabbyState::Fighting => println!("Crabby is fighting"),
            CrabbyState::Collecting(amount) => {
                println!("Crabby is collecting {} treasures", amount)
            }
            CrabbyState::Defending => println!("Crabby is defending"),
        }
    }
}

enum CrabbyState {
    Resting,
    Fighting,
    Collecting(u32),
    Defending,
}

impl fmt::Display for CrabbyState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CrabbyState::Resting => write!(f, "Resting"),
            CrabbyState::Fighting => write!(f, "Fighting"),
            CrabbyState::Collecting(amount) => write!(f, "Collecting {}", amount),
            CrabbyState::Defending => write!(f, "Defending"),
        }
    }
}

struct Inventory<T> {
    item: T,
}

trait DisplayItem {
    fn display(&self);
}

// impl<Generic> Trait for Struct
impl<T> DisplayItem for Inventory<T>
where
    T: fmt::Debug,
{
    fn display(&self) {
        println!("Item: {:?}", self.item); // debug format
    }
}
