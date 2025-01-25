use super::weapons::{Mace, Staff, Sword, Weapon};

pub trait Character {
    fn health_increase(&mut self, value: u8);
    fn health_decrease(&mut self, value: u8);
}

pub struct Warrior {
    pub health: u8,
    strength: u8,
    intelligent: u8,
    pub weapon: Box<dyn Weapon>,
}

impl Warrior {
    pub fn new() -> Self {
        Self {
            health: 100,
            strength: 10,
            intelligent: 0,
            weapon: Box::new(Sword),
        }
    }
}

impl Character for Warrior {
    fn health_increase(&mut self, value: u8) {
        if self.health + value > 100 {
            self.health = 100;
        } else {
            self.health += value;
        }
    }

    fn health_decrease(&mut self, value: u8) {
        self.health = self.health.saturating_sub(value);
    }
}

pub struct Mage {
    pub health: u8,
    strength: u8,
    intelligent: u8,
    pub weapon: Box<dyn Weapon>,
}

impl Mage {
    pub fn new() -> Self {
        Self {
            health: 100,
            strength: 0,
            intelligent: 10,
            weapon: Box::new(Staff),
        }
    }
}

impl Character for Mage {
    fn health_increase(&mut self, value: u8) {
        if self.health + value > 100 {
            self.health = 100;
        } else {
            self.health += value;
        }
    }

    fn health_decrease(&mut self, value: u8) {
        self.health = self.health.saturating_sub(value);
    }
}

pub struct Healer {
    pub health: u8,
    strength: u8,
    intelligent: u8,
    pub weapon: Box<dyn Weapon>,
}

impl Healer {
    pub fn new() -> Self {
        Self {
            health: 100,
            strength: 5,
            intelligent: 5,
            weapon: Box::new(Mace),
        }
    }
}

impl Character for Healer {
    fn health_increase(&mut self, value: u8) {
        if self.health + value > 100 {
            self.health = 100;
        } else {
            self.health += value;
        }
    }

    fn health_decrease(&mut self, value: u8) {
        self.health = self.health.saturating_sub(value);
    }
}

pub fn special_attack(weapon: Box<dyn Weapon>) {
    weapon.attack();
}

pub fn health_increase<T: Character>(character:&mut T, value: u8) {
  character.health_increase(value)
}

pub fn health_decrease<T: Character>(character:&mut T, value: u8) {
  character.health_decrease(value)
}
