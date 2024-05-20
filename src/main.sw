contract;

use std::{
    hash::*, 
    logging::log,
    string::String,
    storage::storage_vec::*,
    storage::storage_string::*, 
};
abi Counter {

    #[storage(read, write)] 
    fn create(username: String);

    #[storage(read)] 
    fn username(vector_index: u64) -> String;

    #[storage(read)] 
    fn total_usernames() -> u64;
}


storage {
    usernames: StorageVec<StorageString> = StorageVec {},
}

impl Counter for Contract {
   
    #[storage(read)] 
    fn total_usernames() -> u64 {storage.usernames.len()}

    #[storage(read)] 
    fn username(vector_index: u64) -> String
    {
        storage.usernames.get(vector_index).unwrap().read_slice().unwrap()
    }

    #[storage(read, write)] 
    fn create(username: String){
        // initialize StorageString Slot 
        storage.usernames.push(StorageString {});
        // get the last ID
        let vector_id = storage.usernames.len() - 1;
        // write_slice for StorageString to the last ID
        storage.usernames.get(vector_id).unwrap().write_slice(username);
    }
}