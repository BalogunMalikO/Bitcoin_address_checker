//We want to create an app that accepts a bitcoin address and spit out what type of address it is with little defination
// we have 4 types of bitcoin address
//1. Pay to public key hash(p2pkh) and starts with "1"
//2. Pay-to-script address and with "3"
//3. segregated witness address that starts with "bc1q"
//4. Taproot address starts with "bc1p"

//logic to implement: Accepts an address, check if it is a bitcoin address and if not spit " 
//not a bitcoin address", if it is a bitcoin address; spit the type it is
extern crate bitcoin;
use bitcoin::Address;
use std::io;
use std::str::FromStr;

fn main() {
    println!("Enter a Bitcoin address: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let res = input.trim();
    let p2pkh = "1";
    let P2sh = "3";
    let P2tr = "bc1p";
    let P2wpkh = "bc1q";
    

    match Address::from_str(res) {
        Ok(_address) => {
            
            match _address.network {
                bitcoin::Network::Bitcoin => { if Address::from_str(res).is_ok() && res.starts_with(p2pkh) {
                    println!("This is a valid Bitcoin address p2pkh - Legacy Address"); 
            
                } else if Address::from_str(res).is_ok() && res.starts_with(P2sh) {
                    println!( "This is a a valid Bitcoin address P2SH - Script Address")
            
                } else if Address::from_str(res).is_ok() && res.starts_with(P2tr) {
                    println!(" This is a valid Bitcoin address P2tr - Taprrot Address")
            
                } else if Address::from_str(res).is_ok() && res.starts_with(P2wpkh){
                    println!(" This is a Valid Bitcoin address P2wpkh - Segwitz Address")
            
                } else {
                    println!(" This is not a valid Bitcoin Address")
                }},
                bitcoin::Network::Testnet => println!("This is a Bitcoin testnet address"),
                bitcoin::Network::Signet =>  println!("This is a Bitcoin signet address"),
                bitcoin::Network::Regtest =>  println!("This is a Bitcoin regtest"),
                _ => println!("error type"),
            };
            
        },
        Err(_) => println!("error"),
        
      
    }



    
   
 

 
     

   
        
 


   

   
}





