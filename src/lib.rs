use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, serde::{Serialize, Deserialize}};
use near_sdk::store::{Vector, UnorderedMap};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    string: String,
    vector: Vector<u64>,
    unordered_map: UnorderedMap<String, String>,
    unordered_map_vec: UnorderedMap<String, Vector<String>>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            string: String::from("Hello World"),
            vector: Vector::new(b"a".to_vec()),
            unordered_map: UnorderedMap::new(b"b".to_vec()),
            unordered_map_vec: UnorderedMap::new(b"c".to_vec()),
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

        Self { string, vector, unordered_map, unordered_map_vec }
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
        assert_eq!(self.unordered_map_vec.is_empty(), true);
        self.unordered_map_vec.insert(key, Vector::new(b"c".to_vec()));
    }

    pub fn unordered_map_vec_insert(&mut self, key: String, insert_value: String) {
        let mut nested = self.unordered_map_vec.get_mut(&key).unwrap();
        nested.push(insert_value);
    }

    // returns unserialized vector
    pub fn unordered_map_vec_get(&self, key: String) -> Option<Vector<String>> {
        if let Some(nested) = self.unordered_map_vec.get(&key) {
            let mut vec = near_sdk::store::Vector::new(b"d".to_vec());
            for element in nested.iter() {
                vec.push(element.clone());
            }
            Some(vec)
        } else {
            None
        }
    }
    
    // returns seriliazed vector as string
    pub fn unordered_map_vec_get_ser(&self, key: String) -> Option<String> {
        if let Some(nested) = self.unordered_map_vec.get(&key) {
            let mut vec: Vec<String> = Vec::with_capacity(nested.len() as usize);
            for element in nested.iter() {
                vec.push(element.clone());
            }
            let serializable_vector = SerializableVector(vec);
            let serialized_vector = serde_json::to_string(&serializable_vector).expect("Serialization error");
            Some(serialized_vector)
        } else {
            None
        }
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
        contract.unordered_map_insert("Account 1".to_string(), "0x1234".to_string());
        assert_eq!(contract.get_unordered_map_value("Account 1".to_string()), "0x1234".to_string());
        contract.unordered_map_insert("Account 1".to_string(), "0xABCD".to_string());
        assert_eq!(contract.get_unordered_map_value("Account 1".to_string()), "0xABCD".to_string())
    }

    #[test]
    #[should_panic]
    fn panic_unordered_map() {
        let mut contract: Contract = Contract::default();
        contract.unordered_map_insert("Account 1".to_string(), "0x1234".to_string());
        assert_eq!(contract.get_unordered_map_value("Account 1".to_string()), "0xABCD".to_string());
    }

    #[test]
    fn test_unordered_map_vec() {
        let mut contract: Contract = Contract::default();
        contract.unordered_map_vec_init("Account 3".to_string());
        
        contract.unordered_map_vec_insert("Account 3".to_string(), "insert 1".to_string());
        println!("{}", contract.unordered_map_vec_get("Account 3".to_string()).unwrap().get(0).unwrap());
        assert_eq!(*(contract.unordered_map_vec_get("Account 3".to_string()).unwrap().get(0).unwrap()), "insert 1".to_string());
        
        contract.unordered_map_vec_insert("Account 3".to_string(), "insert 2".to_string());
        println!("{}", contract.unordered_map_vec_get("Account 3".to_string()).unwrap().get(1).unwrap());
        assert_eq!(*(contract.unordered_map_vec_get("Account 3".to_string()).unwrap().get(1).unwrap()), "insert 2".to_string());
        
        contract.unordered_map_vec_insert("Account 3".to_string(), "insert 3".to_string());
        println!("{}", contract.unordered_map_vec_get("Account 3".to_string()).unwrap().get(2).unwrap());
        assert_eq!(*(contract.unordered_map_vec_get("Account 3".to_string()).unwrap().get(2).unwrap()), "insert 3".to_string());
    }

    #[test]
    fn get_ser_vec() {
        let mut contract: Contract = Contract::default();
        contract.unordered_map_vec_init("Account 3".to_string());
        contract.unordered_map_vec_insert("Account 3".to_string(), "insert 1".to_string());
        contract.unordered_map_vec_insert("Account 3".to_string(), "insert 2".to_string());
        contract.unordered_map_vec_insert("Account 3".to_string(), "insert 3".to_string());
        assert_eq!(contract.unordered_map_vec_get_ser("Account 3".to_string()).unwrap(), "[\"insert 1\",\"insert 2\",\"insert 3\"]");
    }
}