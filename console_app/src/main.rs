use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Bound;
use std::rc::Rc;
use std::sync::mpsc::SyncSender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::ThreadId;
use std::vec;
use std::{fmt, thread};

use my_macro_lib::compute_time;
use rust_101::armors::use_item::use_item;
use rust_101::calculator::add::add;
use rust_101::calculator_2::power::power;
use rust_101::pets::dogs::dog;
use rust_101::potions::drop_item;
use rust_101::{potions, weapons};
// the name `use_item` is defined multiple times
// `use_item` must be defined only once in the type namespace of this module
// use rust_101::shields::use_item::use_item;

use rust_101::shields::use_item;

// need to declare before use
macro_rules! greeting_by_macro {
    ($name:expr) => {
        println!("Hello, {}", $name)
    };
}

macro_rules! magic_spelling {
    (fire) => {
        println!("Fire");
    };
    (water) => {
        println!("Water");
    };
}

// print compute time
#[compute_time]
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
    sample_smart_pointer();
    sample_traits_as_a_type();
    sample_multi_threading();
    sample_share_data_between_threads();
    sample_update_share_data_in_threads();
    sample_channels_for_thread_with_buffer_size();
    sample_channels_for_thread_without_buffer_size();
    sample_channels_for_thread_with_update_shared_data();
    sample_modules_and_crates();
    greeting_by_macro!("World");
    magic_spelling!(fire);
    magic_spelling!(water);
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

fn sample_threat_and_generic() {
    // sample trait & generic
    let gold = Inventory { item: 100 };
    gold.display();

    let armor = Inventory {
        item: "Iron Armor".to_string(),
    };
    armor.display();
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

    let item2 = items
        .get(4)
        .unwrap_or_else(|| &"unwrap_or_else: Item not found");
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

fn sample_smart_pointer() {
    // Box is a smart pointer that allocates memory on the heap
    // Box is used to store data when the size is unknown at compile time
    let root_tree_node = TreeNode {
        name: "Root",
        child: Some(Box::new(TreeNode {
            name: "Child 1",
            child: Some(Box::new(TreeNode {
                name: "Child 2",
                child: None,
            })),
        })),
    };
    println!("Root tree node: {:?}", root_tree_node);

    // Note: normally rust 1 heap can point only 1 owner
    let sword = String::from("Sword");
    // let loot_1 = sword;
    // let loot_2 = sword; // Ownership moved to loot_1, cannot use sword anymore

    // let loot_1 = sword.clone(); // cause of memory performance
    // let loot_2 = sword.clone();

    // Rc is a reference-counted smart pointer that allows multiple owners
    // Rc is used to share data between multiple parts of the program
    // Rc is immutable, cannot modify the value it points to
    let shared_loot = Rc::new(sword);

    let loot_1 = Rc::clone(&shared_loot);
    let loot_2 = Rc::clone(&shared_loot);
    println!("Loot 1: {}", loot_1);
    println!("Loot 2: {}", loot_2);

    // RefCell is a smart pointer that allows mutable borrows checked at runtime
    // RefCell is used to mutate data when the compiler cannot determine if it is mutable
    // RefCell is used with Rc to allow multiple owners to mutate the data
    // RefCell is used when the borrow checker prevents mutable borrows
    let gold = Box::new(10);
    let shared_gold_chest = Rc::new(RefCell::new(gold));
    let gold_chest_1 = Rc::clone(&shared_gold_chest);
    let gold_chest_2 = Rc::clone(&shared_gold_chest);

    **gold_chest_1.borrow_mut() += 10;
    **gold_chest_2.borrow_mut() += 20;

    println!("Gold chest: {}", **shared_gold_chest.borrow());
    // output: Gold chest: 40
    println!("Gold chest 1: {}", **gold_chest_1.borrow());
    // output: Gold chest 1: 40
    println!("Gold chest 2: {}", **gold_chest_2.borrow());
    // output: Gold chest 2: 40

    let bag = 10;
    let shared_bag = Rc::new(RefCell::new(bag));
    let bag_1 = Rc::clone(&shared_bag);
    let bag_2 = Rc::clone(&shared_bag);

    *bag_1.borrow_mut() += 20;
    *bag_2.borrow_mut() += 30;
    *shared_bag.borrow_mut() += 40;

    println!(
        "Shared bag: {}, Bag 1: {}, Bag 2: {}",
        shared_bag.borrow(),
        bag_1.borrow(),
        bag_2.borrow()
    );
}

// Recursive type storing a reference to itself is not allowed
// because rust cannot determine the size of the type at compile time
// recursive type `TreeViewBranch` has infinite size
// struct TreeViewBranch<'a> {
//     name: String,
//     child: Option<TreeViewBranch<'a>>,
// }

#[derive(Debug)]
struct TreeNode<'a> {
    name: &'a str,
    child: Option<Box<TreeNode<'a>>>,
}

// Note:
// use dynamic dispatch for flexibility
// use static dispatch for speed
fn sample_traits_as_a_type() {
    let weapon = get_weapon("sword");
    println!("weapon: {:?}", weapon);
    weapon_attack(weapon);
    // output: Attack by sword"

    let bow = Bow;
    generic_weapon_attack(bow);
    // output: Attack by bow
    let sword = Sword;
    generic_weapon_attack(sword);
    // output: Attach by sword

    let shield = get_shield();
    borrow_shield_block(&shield);
    // output: Defend by wood shield
    shield_block(shield);
    // output: Defend by wood shield
}

trait Weapon: fmt::Debug {
    fn attack(&self);
}

#[derive(Debug)]
struct Sword;

#[derive(Debug)]
struct Bow;

impl Weapon for Sword {
    fn attack(&self) {
        println!("Attack by sword");
    }
}

impl Weapon for Bow {
    fn attack(&self) {
        println!("Attack by bow");
    }
}

// Rust cannot return a trait object directly,
// it must be wrapped in a smart pointer like Box,
// because rust can not estimate memory size
// fn get_weapon(weapon_type: &str) -> Weapon {
//     match weapon_type {
//         "sword" => Box::new(Sword),
//         "bow" => Box::new(Bow),
//         _ => panic!("Unknow weapon type!"),
//     }
// }

// The dyn keyword in Rust is used to indicate that a type is a trait object.
// Trait objects allow for dynamic dispatch,
// which means that the method to be called is determined at runtime rather than compile time.
// This is useful when you want to store different types that implement the same trait in a single collection
// or pass them around as function arguments.
// the dyn keyword can be used with other smart pointers like Rc, Arc, or even directly with references.
// e.g. -> Rc<dyn Weapon> || -> &'a dyn Weapon
// However, Box is commonly used because it allows for heap allocation and ownership transfer,
// which is often needed when working with trait objects
fn get_weapon(weapon_type: &str) -> Box<dyn Weapon> {
    match weapon_type {
        "sword" => Box::new(Sword),
        "bow" => Box::new(Bow),
        _ => panic!("Unknow weapon type!"),
    }
}

// overhead heat memory
fn weapon_attack(weapon: Box<dyn Weapon>) {
    weapon.attack();
}

// safe more than dynamic dispatch
fn generic_weapon_attack<T: Weapon>(weapon: T) {
    weapon.attack();
}

trait Shield {
    fn block(&self);
}

struct WoodShield;

impl Shield for WoodShield {
    fn block(&self) {
        println!("Defend by wood shield");
    }
}

fn get_shield() -> impl Shield {
    WoodShield
}

fn shield_block(shield: impl Shield) {
    shield.block();
}

fn borrow_shield_block(shield: &impl Shield) {
    shield.block();
}

// NOTE: Rust threads, the built-in implementation is 1 Rust Thread :1 Thread in OS
fn sample_multi_threading() {
    let sub_thread = thread::spawn(|| println!("Message from sub_thread"));

    println!("Message from main thread");

    sub_thread.join().unwrap();
    // output: Message from main thread printed first, then Message from sub_thread come after
}

fn sample_share_data_between_threads() {
    let treasure = Arc::new("Fashion Sword");

    let thread_1 = thread::spawn({
        let treasure_clone = Arc::clone(&treasure);

        // move
        // Ensure Consistency: Moving a value guarantees
        //     that you can't mistakenly use data
        //     after it's been transferred else where
        // Prevent Data Races: Only one owner can modify the data at time
        move || {
            println!("1 Found: {}", treasure_clone);
        }
    });

    let thread_2 = thread::spawn({
        let treasure_clone = Arc::clone(&treasure);
        move || {
            println!("2 Found: {}", treasure_clone);
        }
    });

    thread_1.join().unwrap();
    thread_2.join().unwrap();
    // output: 1 Found: Fashion Sword
    // output: 2 Found: Fashion Sword
}

fn sample_update_share_data_in_threads() {
    // Note:
    // use std::sync::Mutex;
    // Mutex (short for mutual exclusion) is a synchronization primitive that provides mutual exclusion
    // allowing only one thread to access the data at a time.
    // It is used to protect shared data from being accessed by multiple threads simultaneously,
    // which can lead to data races and undefined behavior

    // Note:
    // Arc stands for Atomic Reference Counting.
    // It is a thread-safe reference-counting pointer
    // that enables multiple ownership of data.
    // Arc is used when you need to share data between multiple threads
    // and ensure that the data is not deallocated until all references to it are gone.
    let cash = Arc::new(Mutex::new(100));
    let mut handles = vec![];

    for _ in 0..10 {
        let cash = Arc::clone(&cash);
        let handle = thread::spawn(move || {
            let thread_id = thread::current().id();
            let mut store_cash = cash.lock().unwrap();
            *store_cash += 10;

            println!("Thread ID: {:?}, Cash: {}", thread_id, *store_cash);
        });
        handles.push(handle);
    }

    println!("Total Cash: {}", *cash.lock().unwrap());

    for handle in handles {
        handle.join().unwrap();
    }

    let cash_result = cash.lock().unwrap();
    println!("Total Cash: {}", *cash_result);
    // output:
    // Thread ID: ThreadId(5), Cash: 110
    // Total Cash: 110 -> print order randomly
    // Thread ID: ThreadId(6), Cash: 120
    // Thread ID: ThreadId(7), Cash: 130
    // Thread ID: ThreadId(8), Cash: 140
    // Thread ID: ThreadId(9), Cash: 150
    // Thread ID: ThreadId(10), Cash: 160
    // Thread ID: ThreadId(11), Cash: 170
    // Thread ID: ThreadId(12), Cash: 180
    // Thread ID: ThreadId(14), Cash: 190
    // Thread ID: ThreadId(13), Cash: 200
    // Total Cash: 200
}

fn sample_channels_for_thread_with_buffer_size() {
    let items = vec![
        "sword".to_string(),
        "shield".to_string(),
        "potion".to_string(),
        "bow".to_string(),
        "dagger".to_string(),
        "gun".to_string(),
        "hammer".to_string(),
        "axe".to_string(),
        "cloth".to_string(),
    ];

    // Note: sender is called Producer
    let (sender, receiver): (mpsc::SyncSender<String>, mpsc::Receiver<String>) =
        mpsc::sync_channel(items.len());
    let sender_arc = Arc::new(sender);

    for item in items.clone().into_iter() {
        thread::spawn({
            let thread_sender = Arc::clone(&sender_arc);
            let item = item.clone();
            move || {
                thread_sender
                    .send(format!("Worker {}: Task complete!", item))
                    .unwrap();
            }
        });
    }

    // Note: sender is called Consumer
    for _ in 0..items.len() {
        let item = receiver.recv().unwrap();
        println!("Received: {}", item);
    }

    // output: The order of the results depends on which thread finishes its task first.
    // Received: Worker sword: Task complete!
    // Received: Worker potion: Task complete!
    // Received: Worker shield: Task complete!
    // Received: Worker gun: Task complete!
    // Received: Worker dagger: Task complete!
    // Received: Worker bow: Task complete!
    // Received: Worker axe: Task complete!
    // Received: Worker hammer: Task complete!
    // Received: Worker cloth: Task complete!
}

fn sample_channels_for_thread_without_buffer_size() {
    let items = vec![
        "sword".to_string(),
        "shield".to_string(),
        "potion".to_string(),
        "bow".to_string(),
        "dagger".to_string(),
        "gun".to_string(),
        "hammer".to_string(),
        "axe".to_string(),
        "cloth".to_string(),
    ];

    // producer
    let (sender, receiver) = mpsc::channel();
    let sender_arc = Arc::new(sender);

    for item in items.clone().into_iter() {
        thread::spawn({
            let thread_sender = Arc::clone(&sender_arc);
            let item = item.clone();
            move || {
                thread_sender
                    .send(format!("Worker {}: Task complete!", item))
                    .unwrap();
            }
        });
    }

    // Dropping all senders signals to the receiver that no more messages will be sent.
    drop(sender_arc);

    // consumer
    while let Ok(item) = receiver.recv() {
        println!("Received (without buffer size): {}", item)
    }

    // output: The order of the results depends on which thread finishes its task first.
    // Received (without buffer size): Worker sword: Task complete!
    // Received (without buffer size): Worker shield: Task complete!
    // Received (without buffer size): Worker potion: Task complete!
    // Received (without buffer size): Worker dagger: Task complete!
    // Received (without buffer size): Worker bow: Task complete!
    // Received (without buffer size): Worker gun: Task complete!
    // Received (without buffer size): Worker hammer: Task complete!
    // Received (without buffer size): Worker cloth: Task complete!
    // Received (without buffer size): Worker axe: Task complete!
}

fn sample_channels_for_thread_with_update_shared_data() {
    let loots_gold = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let mut gold = 100;

    let (sender, receiver): (
        mpsc::SyncSender<(ThreadId, i32)>,
        mpsc::Receiver<(ThreadId, i32)>,
    ) = mpsc::sync_channel(3);
    let sender_arc = Arc::new(sender);

    for loot_gold in loots_gold.into_iter() {
        thread::spawn({
            let sender = Arc::clone(&sender_arc);
            move || {
                let data = (thread::current().id(), loot_gold);
                sender.send(data).unwrap();
            }
        });
    }

    drop(sender_arc);

    // for received in receiver {
    //     let (thread_id, loot_gold) = received;
    //     gold += loot_gold;
    //     println!("Received from thread_id: {:?}, loot_gold: {}", thread_id, loot_gold);
    // }

    loop {
        match receiver.recv() {
            Ok((thread_id, loot_gold)) => {
                gold += loot_gold;

                println!(
                    "Received from thread_id: {:?}, loot_gold: {}",
                    thread_id, loot_gold
                );
            }
            Err(_) => {
                println!("All senders have been dropped");
                break;
            }
        }
    }

    println!("gold: {}", gold);

    // output:
    // Received from thread_id: ThreadId(33), loot_gold: 10
    // Received from thread_id: ThreadId(34), loot_gold: 20
    // Received from thread_id: ThreadId(35), loot_gold: 30
    // Received from thread_id: ThreadId(36), loot_gold: 40
    // Received from thread_id: ThreadId(40), loot_gold: 80
    // Received from thread_id: ThreadId(38), loot_gold: 60
    // Received from thread_id: ThreadId(39), loot_gold: 70
    // Received from thread_id: ThreadId(37), loot_gold: 50
    // Received from thread_id: ThreadId(42), loot_gold: 100
    // Received from thread_id: ThreadId(41), loot_gold: 90
    // All senders have been dropped
    // gold: 650
}

fn sample_modules_and_crates() {
    potions::use_item();
    weapons::use_item();
    maps::use_item();
    drop_item();
    use_item();
    use_item::use_item();
    dog::bark();

    println!("1 + 2 = {}", add(1, 2));
    println!("2^3 = {}", power(2, 3));
}

mod maps {
    pub fn use_item() {
        println!("Use a map");
    }
}
