
use solana_sdk::{pubkey::Pubkey, system_instruction, transaction::Transaction};
#[allow(unused_imports)]
use solana_sdk::{ self, pubkey, signer::{ keypair::Keypair, Signer },signature::Signature };
use solana_client::rpc_client::RpcClient;
use std::{io::stdin, str::FromStr};

#[allow(unused_variables)]

#[tokio::main]
#[allow(unused_mut)]
async fn main() {
    let url = "https://api.devnet.solana.com";
    // Conncetting the Solana Cluster.
    let client = RpcClient::new(url.to_string());
    println!(" ======================= Welcome to solana CLI Rpc is connected to devnet by default =======================");
    let mut public_key;


 let mut public_intiate = String::new();
 println!("Do you have public key ? (yes or no)");
 stdin().read_line(&mut public_intiate).expect("Inavlid input");
 let public_intiate = public_intiate.trim();

// Public key initiation
 match public_intiate == "yes" {
     true =>{
        println!("Paste your Public-Key");
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Invalid input");
            let input = input.trim();
             public_key = match Pubkey::from_str(input) {
                 Ok(pubkey)=>pubkey,
                 Err(e)=>panic!("{e}")
             };
     }
     false=>{
        println!("Generating your keypair......");
            let key_pair = create_key_pair();
            println!("Your Keypair is generated ✅");
            public_key = key_pair.pubkey();
            println!("Your public key is : {public_key}");
         }
 }

 loop {
    println!("You can choose the options:");
    println!("1.Click if you want airdrop solana");
    println!("2.Click if you check current balance");
    println!("3.Click if you want to exit");
   
    let mut option = String::new();
    stdin().read_line(&mut option).expect("Invalid input");
    let option:i32 = option.trim().parse().expect("Invalid option");
   
   
       match option {
           1=>{
               println!("How many sol's you want to airdrop ?(0-1 range)");
               let mut input = String::new();
               stdin().read_line(&mut input).expect("Invalid input");
               let input:u64 = input.trim().parse().expect("Invalid number");
               let lamports = input*1_000_000_000;
               println!("Requsting for airdrop......");
               req_airdrop(&client, &public_key);
           },
           2=>{
               println!("Fetching your balance...");
               check_account_balance(&client,&public_key);
           },
           3=>{
            break;
           },
           _=>{
            println!("Invalid option");
           }
       }
   
 }

}

#[allow(dead_code)]
    // Generate Keypair
fn create_key_pair() -> Keypair {
    let key_pair = Keypair::new();
    // println!("The public key is {:?} in function",key_pair.pubkey());
    key_pair
}

#[allow(dead_code)]
fn req_airdrop(client:&RpcClient,public_key:&Pubkey){
   let response  =  client.request_airdrop(&public_key, 1_000_000_000);
   match response {
       Ok(data)=>println!("Airdrop is successfull hear your sign : {data}"),
       Err(e)=>println!("{e}")
   }
}

#[allow(dead_code)]
fn check_account_balance(client:&RpcClient,public_key:&Pubkey){
    let balanace = client.get_balance(public_key);
    match balanace {
        Ok(data)=>{
            let sol_balance:f64 = data as f64/1_000_000_000 as f64;
            println!("The bank balance is: {sol_balance}")
        },
        Err(e) => println!("{e}")
    }
}

#[allow(dead_code)]
fn lamports_transaction(client:&RpcClient,sender:&Keypair,recipt_address:Pubkey,amount:u64){
    let block_hash_recent = client.get_latest_blockhash().unwrap();

    // Create Instructions
    let transfer_instruction = system_instruction::transfer(
          &sender.pubkey(),
          &recipt_address,
          amount
        );

    //Create and sign the transaction
    let transaction = Transaction::new_signed_with_payer(
          &[transfer_instruction], 
          Some(&sender.pubkey()),
          &[sender], 
          block_hash_recent
        );

    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature)=>println!(
            "{amount} is successfully transferd to {recipt_address} 🥳🥳, Signature: {signature}"
        ),
        Err(e)=>println!("{e}")
    }
}