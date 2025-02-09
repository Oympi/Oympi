use uuid::Uuid;

pub struct Message {
    message_id: u32,
    content: Vec<u8>,
    sender_pubkey: String,
    receiver_pubkey: String,
    signature: Vec<u8>,
    delete_at: Vec<u16>
}

impl Message {
    pub fn new(sender_pubkey: Vec<u8>, receiver_pubkey: Vec<u8>, content: String,delete_after: Vec<u16>)-> Self {
        let uuid = Uuid::new_v4();
        Self {
            message_id: uuid,
            content: content,
            sender_pubkey,
            receiver_pubkey,
            signature,
            delete_at: delete_after,
        }
        
    }
}