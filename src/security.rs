pub mod security{
    use ed25519_dalek::SigningKey;

    pub fn generate_keypair() -> (Vec<u8>, Vec<u8>){
            todo!()
    }

    pub fn encrypt(message: String, receiver_pubkey: &Vec<u8>)-> Vec<u8>{
        todo!()
    }
    pub fn decrypt(encrypted_message: Vec<u8>, private_key:&Vec<u8>)->String{
        todo!()
    }
    pub fn sign_message(message: &Vec<u8>, private_key:&Vec<u8>)->Vec<u8>{
        todo!()
    }
    pub fn verify_signature(message: &Vec<u8>,signature: &Vec<u8>, sender_pubkey:&Vec<u8>)->bool{
        todo!()
    }
}