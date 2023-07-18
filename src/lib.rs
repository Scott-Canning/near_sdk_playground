use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, serde::{Serialize, Deserialize}};
use near_sdk::store::{Vector, UnorderedMap};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    string: String,
    vector: Vector<u64>,
    unordered_map: UnorderedMap<String, String>,
    unordered_map_vec: UnorderedMap<String, Vector<String>>,
    unordered_map_map: UnorderedMap<String, UnorderedMap<String, Vector<String>>>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            string: String::from("Hello World"),
            vector: Vector::new(b"a".to_vec()),
            unordered_map: UnorderedMap::new(b"b".to_vec()),
            unordered_map_vec: UnorderedMap::new(b"c".to_vec()),
            unordered_map_map: UnorderedMap::new(b"d".to_vec()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableVector(Vec<String>);

#[near_bindgen]
impl Contract {
    #[init]
    #[private]
    pub fn init(string: String, first_value: u64) -> Self {
        let mut vector: Vector<u64> = Vector::new(b"a".to_vec());
        vector.push(first_value);
        let mut unordered_map: UnorderedMap<String, String> = UnorderedMap::new(b"b".to_vec());
        let mut unordered_map_vec: UnorderedMap<String, Vector<String>> = UnorderedMap::new(b"c".to_vec());
        let mut unordered_map_map: UnorderedMap<String, UnorderedMap<String, Vector<String>>> = UnorderedMap::new(b"d".to_vec());

        Self { string, vector, unordered_map, unordered_map_vec, unordered_map_map }
  }

    pub fn change_string_vec(&mut self, string: String, number: u64) {
        self.string = string;
        self.vector.push(number);
    }

    pub fn change_string(&mut self, string: String) {
        self.string = string;
    }

    pub fn get_string(&self) -> String {
        return self.string.clone();
    }

    pub fn vector_push(&mut self, number: u64) {
        self.vector.push(number);
    }

    pub fn get_vector_index(&self, index: u32) -> u64 {
        return *(self.vector.get(index).unwrap());
    }

    // UnorderedMap<String, String>
    pub fn unordered_map_insert(&mut self, key: String, value: String) {
        self.unordered_map.insert(key, value);
    }

    pub fn get_unordered_map_value(&self, key: String) -> String {
        return self.unordered_map.get(&key).unwrap().to_string();
    }

    // UnorderedMap<String, Vector<String>>
    pub fn unordered_map_vec_init(&mut self, key: String) {
        let caller: near_sdk::AccountId = env::signer_account_id();
        assert_eq!(caller.to_string(), key, "Only owner");
        let nested = self.unordered_map_vec.get_mut(&key);
        if nested.is_none() {
            self.unordered_map_vec.insert(key, Vector::new(b"c".to_vec()));
        } else {
            return;
        }
    }

    pub fn unordered_map_vec_insert(&mut self, key: String, insert_value: String) {
        let caller: near_sdk::AccountId = env::signer_account_id();
        assert_eq!(caller.to_string(), key, "Only owner");
        let mut nested = self.unordered_map_vec.get_mut(&key).unwrap();
        nested.push(insert_value);
    }

    pub fn unordered_map_vec_extend(&mut self, key: String, insert_values: std::vec::Vec<String>) {
        let caller: near_sdk::AccountId = env::signer_account_id();
        assert_eq!(caller.to_string(), key, "Only owner");
        let mut nested = self.unordered_map_vec.get_mut(&key).unwrap();
        nested.extend(insert_values);
    }
    
    // returns seriliazed vector as string
    pub fn unordered_map_vec_get_ser(&self, key: String) -> String {
        let nested = self.unordered_map_vec.get(&key).unwrap(); 
        let mut vec: Vec<String> = Vec::with_capacity(nested.len() as usize);
        for element in nested.iter() {
            vec.push(element.clone());
        }
        let serializable_vector = SerializableVector(vec);
        let serialized_vector = serde_json::to_string(&serializable_vector).expect("Serialization error");
        serialized_vector
    }

    pub fn unordered_map_map_int(&mut self, key: String) {
        self.unordered_map_map.insert(key, UnorderedMap::new(b"d".to_vec()));
        // LEFT OFF
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_string_vec() {
        let mut contract: Contract = Contract::default();        
        contract.change_string_vec(String::from("Good morning"), 128);
        assert_eq!(contract.get_string(), "Good morning");
        assert_eq!(contract.get_vector_index(0), 128);
    }

    #[test]
    fn test_vector_push() {
        let mut contract: Contract = Contract::default();
        contract.vector_push(256);
        assert_eq!(contract.get_vector_index(0), 256);
        contract.vector_push(512);
        assert_eq!(contract.get_vector_index(1), 512);
        contract.vector_push(1024);
        assert_eq!(contract.get_vector_index(2), 1024);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn panc_vector_get_index() {
        let contract: Contract = Contract::default();
        assert_eq!(contract.get_vector_index(0), 256);
    }

    #[test]
    fn test_unordered_map() {
        let mut contract: Contract = Contract::default();
        contract.unordered_map_insert("bob.near".to_string(), "0x1234".to_string());
        assert_eq!(contract.get_unordered_map_value("bob.near".to_string()), "0x1234".to_string());
        contract.unordered_map_insert("bob.near".to_string(), "0xABCD".to_string());
        assert_eq!(contract.get_unordered_map_value("bob.near".to_string()), "0xABCD".to_string())
    }

    #[test]
    #[should_panic]
    fn panic_unordered_map() {
        let mut contract: Contract = Contract::default();
        contract.unordered_map_insert("bob.near".to_string(), "0x1234".to_string());
        assert_eq!(contract.get_unordered_map_value("bob.near".to_string()), "0xABCD".to_string());
    }

    #[test]
    fn test_map_vec() {
        let mut contract: Contract = Contract::default();
        contract.unordered_map_vec_init("bob.near".to_string());
        contract.unordered_map_vec_insert("bob.near".to_string(), "insert 1".to_string());
        contract.unordered_map_vec_insert("bob.near".to_string(), "insert 2".to_string());
        contract.unordered_map_vec_insert("bob.near".to_string(), "insert 3".to_string());
        assert_eq!(contract.unordered_map_vec_get_ser("bob.near".to_string()), "[\"insert 1\",\"insert 2\",\"insert 3\"]");
    }

    #[test]
    #[should_panic]
    fn panic_map_vec_init() {
        let mut contract: Contract = Contract::default();
        contract.unordered_map_vec_init("alice.near".to_string());
    }

    #[test]
    #[should_panic]
    fn panic_map_vec_insert() {
        let mut contract: Contract = Contract::default();
        contract.unordered_map_vec_init("bob.near".to_string());
        contract.unordered_map_vec_insert("alice.near".to_string(), "insert 1".to_string());
    }

    #[test]
    fn test_unordered_map_vec_extend() {
        let mut contract: Contract = Contract::default();
        contract.unordered_map_vec_init("bob.near".to_string());
        contract.unordered_map_vec_insert("bob.near".to_string(), "0xFirstValue".to_string());
        let vec: Vec<String> = vec!["0xSecondValue".to_string(), "0xThirdValue".to_string(), "0xFourthValue".to_string()];
        contract.unordered_map_vec_extend("bob.near".to_string(),vec );
        assert_eq!(contract.unordered_map_vec_get_ser("bob.near".to_string()), "[\"0xFirstValue\",\"0xSecondValue\",\"0xThirdValue\",\"0xFourthValue\"]");
    }
}