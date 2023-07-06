use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};
use near_sdk::collections::{Vector, UnorderedMap};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    string: String,
    vector: Vector<u64>,
    map: UnorderedMap<String, String>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            string: String::from("Hello World"),
            vector: Vector::new(b"v".to_vec()),
            map: UnorderedMap::new(b"l".to_vec()),
        }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    #[private]
    pub fn init(string: String, first_value: u64) -> Self {
        let mut vector: Vector<u64> = Vector::new(b"v".to_vec());
        vector.push(&first_value);
        let mut map: UnorderedMap<String, String> = UnorderedMap::new(b"l".to_vec());
        Self { string, vector, map }
  }

    pub fn change_string_vec(&mut self, string: String, number: u64) {
        self.string = string;
        self.vector.push(&number);
    }

    pub fn change_string(&mut self, string: String) {
        self.string = string;
    }

    pub fn get_string(&self) -> String {
        return self.string.clone();
    }

    pub fn vector_push(&mut self, number: u64) {
        self.vector.push(&number);
    }

    pub fn get_vector_index(&self, index: u64) -> u64 {
        return self.vector.get(index).unwrap();
    }

    pub fn map_insert(&mut self, key: String, value: String) {
        self.map.insert(&key, &value);
    }

    pub fn get_map_value(&self, key: String) -> Option<String> {
        return self.map.get(&key);
    }
}