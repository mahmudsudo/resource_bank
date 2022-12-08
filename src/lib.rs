use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen,collections::LookupMap};



// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    resource: LookupMap<String,String>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{resource: LookupMap::new(b"m")}
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Public method - returns the resource saved
    pub fn get_resource(&self,index  :String) -> String {
        return self.resource.get(&index).unwrap().clone();
    }

    // Public method - accepts a resource
    pub fn set_resource(&mut self, CID: String,index: String) {
        // Use env::log to record logs permanently to the blockchain!
        assert_eq!(self.resource.contains_key(&index),false);
        self.resource.insert(&index,&CID);
        log!("Saving resource {}", CID);

    }
}

