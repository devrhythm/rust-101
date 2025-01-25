pub trait Weapon {
  fn attack(&self);
}

pub struct Sword;
pub struct Staff;
pub struct Mace;

impl Weapon for Sword {
  fn attack(&self) {
    println!("Sword Attack");
  }
}

impl Weapon for Staff {
  fn attack(&self) {
    println!("Staff Attack");
  }
}

impl Weapon for Mace {
  fn attack(&self) {
    println!("Mace Attack");
  }
}
