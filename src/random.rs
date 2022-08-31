use rand::{thread_rng, Rng};

pub fn random_number(length: usize) -> usize {
  let mut rng = thread_rng();
  rng.gen_range(0..length)
}