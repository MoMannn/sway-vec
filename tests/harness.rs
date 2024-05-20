use fuels::{prelude::*, types::ContractId};

// Load abi from json
abigen!(Contract(
    name = "MyContract",
    abi = "out/debug/counter-contract-abi.json"
));

async fn get_contract_instance() -> (MyContract<WalletUnlocked>, ContractId) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await
    .unwrap();
    let wallet = wallets.pop().unwrap();

    let id = Contract::load_from(
        "./out/debug/counter-contract.bin",
        LoadConfiguration::default(),
    )
    .unwrap()
    .deploy(&wallet, TxPolicies::default())
    .await
    .unwrap();

    let instance = MyContract::new(id.clone(), wallet);

    (instance, id.into())
}


async fn create_element(instance: &MyContract<WalletUnlocked>, username: String) -> String {

    let result_created = instance.methods()
    .create(username.clone())
    .call().await;
    
    match result_created {
        Ok(_) => {
            println!("{}  {:?} \n", "username created", username);
            format!("created {:?}", result_created.unwrap().value)
        }
        Err(error) => {
            error.to_string()
        }
    }
}

async fn total_usernames(instance: &MyContract<WalletUnlocked>) {
    
    let response = instance.methods().total_usernames().call().await;
    match response {
        Ok(_) => { println!("{}  {:?} \n", "total_usernames", response.unwrap().value); }
        Err(_) => { println!("total_usernames Error {:?} \n", response.err()); }
    }
}

async fn get_a_username(instance: &MyContract<WalletUnlocked>, index: u64) {
    let response = instance.methods().username(index.clone()).call().await;
    match response {
        Ok(_) => { println!("{} {} {:?} \n", "username at vector_index:", index, response.unwrap().value); }
        Err(_) => { println!("{:?} \n", response.err()); }
    }
}

#[tokio::test]
async fn test_vector() {
    let (instance, _id) = get_contract_instance().await;
    

    let usr1 = ("primoz".to_string(), "primoz@mail.com".to_string());
    let usr2 = ("marko".to_string(), "marko@mail.com".to_string());
    let usr3 = ("jure".to_string(), "jure@mail.com".to_string());
    let usr4 = ("tine".to_string(), "tine@mail.com".to_string());
    let _usr5 = ("nina".to_string(), "nina@mail.com".to_string());

    total_usernames(&instance).await;
    create_element(&instance, usr1.0.clone()).await;
    get_a_username(&instance, 0).await;
    
    println!("{:?} \n", "!!! As we keep pushing, all the StorageVec fields have the same value!!!");
    total_usernames(&instance).await;
    create_element(&instance, usr2.0.clone()).await;
    get_a_username(&instance, 0).await;
    get_a_username(&instance, 1).await;

    total_usernames(&instance).await;
    create_element(&instance, usr3.0.clone()).await;
    get_a_username(&instance, 0).await;
    get_a_username(&instance, 1).await;
    get_a_username(&instance, 2).await;

    total_usernames(&instance).await;
    create_element(&instance, usr4.0.clone()).await;

    
    get_a_username(&instance, 0).await;
    get_a_username(&instance, 1).await;
    get_a_username(&instance, 2).await;
    get_a_username(&instance, 3).await;
}