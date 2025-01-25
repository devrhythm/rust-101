// super refers to the root of the module
use super::mul::mul;

pub fn power(a: i32, b: i32) -> i32 {
  let mut result = 1;

  for _ in 0..b {
    result = mul(result, a);
  }

  result
}
