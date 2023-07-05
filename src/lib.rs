use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};
use near_sdk::collections::{Vector};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
  string: String,
  vector: Vector<u64>
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            string: String::from("Hello World"),
            vector: Vector::new(b"m".to_vec()),
        }
    }
}


#[near_bindgen]
impl Contract {
  #[init]
  pub fn init(string: String, first_value: u64) -> Self {
    let mut vector: Vector<u64> = Vector::new("prefix".as_bytes());
    vector.push(&first_value);

    Self { string, vector }
  }

  pub fn change_state(&mut self, string: String, number: u64) {
    self.string = string;
    self.vector.push(&number);
  }

  pub fn change_string(&mut self, string: String) {
    self.string = string;
  }

  pub fn vector_push(&mut self, number: u64) {
    self.vector.push(&number);
  }

  pub fn get_string(&self) -> String {
    return self.string.clone();
  }

  pub fn get_vector_index(&self, index: u64) -> u64 {
    return self.vector.get(index).unwrap()
  }
}