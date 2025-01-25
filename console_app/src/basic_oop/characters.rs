use super::weapons::{Mace, Staff, Sword, Weapon};

pub struct Warrior {
  pub health: u8,
  strength: u8,
  intelligent: u8,
  pub weapon: Box<dyn Weapon>
}

impl Warrior {
  pub fn new() -> Self {
    Self {
      health: 100,
      strength: 10,
      intelligent: 0,
      weapon: Box::new(Sword)
    }
  }

  pub fn health_increase(&mut self, value: u8) {
    if self.health + value > 100 {
      self.health = 100;
    } else {
      self.health += value;
    }
  }

  pub fn health_decrease(&mut self, value: u8) {
    self.health = self.health.saturating_sub(value);
  }
}

pub struct Mage {
  pub health: u8,
  strength: u8,
  intelligent: u8,
  pub weapon: Box<dyn Weapon>
}

impl Mage {
  pub fn new() -> Self {
    Self {
      health: 100,
      strength: 0,
      intelligent: 10,
      weapon: Box::new(Staff)
    }
  }

  pub fn health_increase(&mut self, value: u8) {
    if self.health + value > 100 {
      self.health = 100;
    } else {
      self.health += value;
    }
  }

  pub fn health_decrease(&mut self, value: u8) {
    self.health = self.health.saturating_sub(value);
  }
}

pub struct Healer {
  pub health: u8,
  strength: u8,
  intelligent: u8,
  pub weapon: Box<dyn Weapon>
}

impl Healer {
  pub fn new() -> Self {
    Self {
      health: 100,
      strength: 5,
      intelligent: 5,
      weapon: Box::new(Mace)
    }
  }

  pub fn health_increase(&mut self, value: u8) {
    if self.health + value > 100 {
      self.health = 100;
    } else {
      self.health += value;
    }
  }

  pub fn health_decrease(&mut self, value: u8) {
    self.health = self.health.saturating_sub(value);
  }
}

pub fn special_attack(weapon: Box<dyn Weapon>) {
  weapon.attack();
}
