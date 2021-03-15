use signer::key_generate_mnemonic;
fn main() {
    let mne = key_generate_mnemonic();
    match mne{
        Ok(m)=>{
            println!("ok");

        },
        Err(err)=>{
            println!("err");

        }
    }
}