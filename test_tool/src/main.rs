use signer::{key_generate_mnemonic, derive_extended_secret_key_from_mnemonic};
fn main(){
    // display_mnemonic()
    // secret_key_from_mnemonic()
    // get_key_derive()
    transaction_sign_bls()
}


///显示助记词
fn display_mnemonic(){
    let mne = key_generate_mnemonic();
    match mne{
        Ok(word)=>{
            println!("result:{:?}",word.0);
        },
        Err(err)=>{
            println!("err:{:?}",err);
        }
    }
}

///生成私钥
fn secret_key_from_mnemonic(){
    let mnemonic = String::from("predict course total rifle tip sense want deer youth nose town frog");
    let path = String::from("m/44'/60'/0'/0/0");
    let password = String::from("123456");
    let lang_code = String::from("en");
    let secret = derive_extended_secret_key_from_mnemonic(&*mnemonic, &*path, &*password, &*lang_code);
    match secret {
        Ok(s)=>{
            println!("{:?}",s)
        },
         Err(err)=>{
              println!("err:{:?}",err);
         }
    }
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
