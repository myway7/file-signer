use signer::{key_generate_mnemonic, derive_extended_secret_key_from_mnemonic,
             seed_from_mnemonic,master_from_seed,key_derive,transaction_sign_raw};
use signer::error::SignerError;
use signer::extended_key::ExtendedSecretKey;
use signer::api::UnsignedMessageAPI;
use std::convert::TryFrom;
use signer::*;
use signer::signature::Signature;

fn main(){
    // generate_mnemonic()
    // secret_key_from_mnemonic()
    // get_key_derive()
    // transaction_sign_bls()
    // master()
    // secret_key()
    transaction_sign_bls()
}

///the first step
///generate mnemonic
fn generate_mnemonic(){
    let mne = key_generate_mnemonic().expect("could not genarate mnemonic");
    println!("{:?}",mne.0);
    // match mne{
    //     Ok(word)=>{
    //         println!("result:{:?}",word.0);
    //     },
    //     Err(err)=>{
    //         println!("err:{:?}",err);
    //     }
    // }
}

///the second step
///mnemonic -> seed
fn seed(){
    println!("{:?}",seed_from_mnemonic())

}

///the third step
/// seed -> master
fn master(){
   let master =  master_from_seed(seed_from_mnemonic().as_bytes()).unwrap();
    println!("{:?}",master);
    println!("{:?}",hex::encode(master.secret_key()).to_string());
    println!("{:?}",hex::encode(master.chain_code()).to_string())
}

///生成私钥
fn secret_key(){
    let mnemonic = String::from("predict course total rifle tip sense want deer youth nose town frog");
    let path = String::from("m/44'/461'/0'/0/0");
    let password = String::from("123456");
    let lang_code = String::from("en");
    let secret = key_derive(&*mnemonic, &*path, &*password, &*lang_code).unwrap();
    println!("{}",secret.address);
    println!("{:?}",hex::encode(secret.private_key.0));
}

///bls签名
fn transaction_sign_bls() {
    // "bls_private_key": "P2pSgkvsZSgi0LOczuHmSXT1+l/hvSs3fVBb4y8OgVo=",
    // "bls_address": "t3uxb75vcy3ilwbsaavao52v7gfnfh6aics4a7nj26dwpcmj4mxxgnzholkupuplafdrbd55frpoolfnm7wlda",
    // "private_key": "8VcW07ADswS4BV2cxi5rnIadVsyTDDhY1NfDH19T8Uo="
    // Prepare message with BLS address
    let bls_address = "t3uxb75vcy3ilwbsaavao52v7gfnfh6aics4a7nj26dwpcmj4mxxgnzholkupuplafdrbd55frpoolfnm7wlda";
    let bls_key = PrivateKey::try_from("8VcW07ADswS4BV2cxi5rnIadVsyTDDhY1NfDH19T8Uo=".to_string()).unwrap();
    let message = UnsignedMessageAPI {
        to: "t17uoq6tp427uzv7fztkbsnn64iwotfrristwpryy".to_string(),
        from: bls_address.to_string(),
        nonce: 1,
        value: "100000".to_string(),
        gas_limit: 25000,
        gas_fee_cap: "2500".to_string(),
        gas_premium: "2500".to_string(),
        method: 0,
        params: "".to_string(),
    };
    let raw_sig = transaction_sign_raw(&message,&bls_key).unwrap();
    let res = CborBuffer(raw_sig.as_bytes());
}
