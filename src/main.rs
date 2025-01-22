use std::collections::HashMap;
use std::fmt;
use std::vec;

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
    sample_error_handling();
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
    let quests = vec![
        "Find the treasure",
        "Defeat the dragon",
        "Save the princess",
    ];

    // closure is inline function
    let completed_quests: Vec<String> = quests
        .iter()
        .map(|quest| format!("Completed: {}", quest))
        .collect();

    println!("completed_quests: {:?}", completed_quests);

    let add = |a, b| a + b;
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

fn sample_error_handling() {
    let items = vec!["Sword", "Shield", "Potion", "Gold"];

    match items.get(4) {
        Some(item) => println!("Item: {}", item),
        None => println!("Item not found"),
    }
    // output: Item not found

    let item = items.get(4).unwrap_or(&"unwrap_or: Item not found");
    println!("Item: {}", item);
    // output: Item: Item not found

    let item2 = items.get(4).unwrap_or_else(|| &"unwrap_or_else: Item not found");
    println!("Item: {}", item2);
    // unwrap_or_else is used to provide a custom error message

    // let item3 = items.get(4).unwrap(); // panicked called `Option::unwrap()` on a `None` value


    let treasure = open_treasure("correct-key");
    match treasure {
        Ok(value) => println!("Chest 1: Success: {}", value),
        Err(error) => println!("Chest 1: Error: {}", error),
    }
    // output: Treasure: You got 10$

    let chest_result = match open_treasure("incorrect-key") {
        Ok(message) => message,
        Err(error) => error,
        // Err(error) => panic!("Error: {}", error),
        // panic! macro is used to stop the program and print an error message
    };
    println!("Chest 2 Result: {}", chest_result);
    // output: Error: Treasure chest is locked

    if let Ok(treasure) = open_treasure("correct-key") {
        println!("Chest 3: Success: {}", treasure);
    } else {
        println!("Chest 3: Invalid key");
    }

    let _ = open_treasure_propagating_errors_with_question_mark("correct-key");
    // output: Chest 4: You got 10$

    let chest5_result = open_treasure_propagating_errors_with_question_mark("incorrect-key");
    if let Err(error) = chest5_result {
        println!("Chest 5: Error: {}", error);
    }
    // output: Error: Invalid key

    let upgrade_weapon_result = match upgrade_weapon("Sword", 10) {
        Some(message) => message,
        None => "Not enough materials".to_string(),
    };

    println!("Upgrade weapon 1: {}", upgrade_weapon_result);
    // output: Upgrade weapon: sword upgraded successfully

    let upgrade_weapon_result = match upgrade_weapon("Bow", 5) {
        Some(message) => message,
        None => "Not enough materials".to_string(),
    };
    println!("Upgrade weapon 2: {}", upgrade_weapon_result);


}

fn open_treasure(key: &str) -> Result<String, String> {
    if key == "correct-key" {
        Ok("You got 10$".to_string())
    } else {
        Err("Invalid key".to_string())
    }
}

fn open_treasure_propagating_errors_with_question_mark(key: &str) -> Result<(), String> {
    // ? operator is used to propagate errors
    // It can only be used in functions that return Result or Option
    // It returns the value inside Ok or stops the function and returns the error inside Err
    let chest_result = open_treasure(key)?;

    println!("Chest 4: {}", chest_result);

    // () is unit type, used when the function does not return a value
    Ok(())
}

fn upgrade_weapon(weapon: &str, material_count: i32) -> Option<String> {
    if material_count >= 10 {
       Some(format!("{} upgraded successfully", weapon))
    } else {
        None
    }
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
