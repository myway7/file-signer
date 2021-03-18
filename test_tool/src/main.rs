use signer::{key_generate_mnemonic, derive_extended_secret_key_from_mnemonic,
             seed_from_mnemonic,master_from_seed,key_derive};
use signer::error::SignerError;
use signer::extended_key::ExtendedSecretKey;

fn main(){
    // generate_mnemonic()
    // secret_key_from_mnemonic()
    // get_key_derive()
    // transaction_sign_bls()
    // master()
    // secret_key()
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
// const privateKeyBase64 = "NZrLSD1pztRBFooiFK9JutcJ0Hlb1FAAP2tNStpfQk4=";
// const message = {
//     Version: 0,
//     To: "f01007",
//     From:
//     "f3wlgsim745hmsyo73egqmpszst4lo5fsppevvvkfyn3dmaqsjecgtdcxhz52xz2drk73ouuxpngegn6il3aja",
//     Nonce: 1,
//     Value: "0",
//     GasLimit: 1990041,
//     GasFeeCap: "100851",
//     GasPremium: "99797",
//     Method: 23,
//     Params: "QwD0Bw==",
// };
#[derive( Debug)]
struct Message {
     to: String,
     from: String,
     nonce: u64,
     value: String,
     gas_limit: i64,
     gas_fee_cap: String,
     gas_premium: String,
     method: u64,
     params: String,
}
fn transaction_sign_bls(){
    let privateKeyBase64 = String::from("NZrLSD1pztRBFooiFK9JutcJ0Hlb1FAAP2tNStpfQk4=");

    let mesage = Message{
        to:String::from("f01007"),
        from:String::from("f3wlgsim745hmsyo73egqmpszst4lo5fsppevvvkfyn3dmaqsjecgtdcxhz52xz2drk73ouuxpngegn6il3aja"),
        nonce:1,
        value: String::from("0"),
        gas_limit:1990041,
        gas_fee_cap:String::from("100851"),
        gas_premium:String::from("99797"),
        method:23,
        params:String::from("QwD0Bw=="),
    };
    println!("{:?}",mesage);


}
