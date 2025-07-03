use ed25519_dalek::Signature;

#[derive(Debug)]
pub struct Transaction{
    sender : String,
    receiver : String,
    amount : f32,
    fee : f32,
    signature : Option<Signature>
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: f32, fee: f32) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
            fee,
            signature : None
        }
    }

    pub fn set_signature(&mut self, signature : Signature) {
        let _ = self.signature.insert(signature);
    }

    pub fn get_sender(&self) -> &String {
        &self.sender
    }

    pub fn get_receiver(&self) -> &String {
        &self.receiver
    }

    pub fn get_amount(&self) -> f32 {
        self.amount
    }

    pub fn get_fee(&self) -> f32 {
        self.fee
    }

    pub fn get_signature(&self) -> Option<&Signature> {
        self.signature.as_ref()
    }

    pub fn serialize(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}",
            self.sender,
            self.receiver,
            self.amount,
            self.fee,
            match self.signature {
                Some(signature) => hex::encode(signature.to_bytes()),
                None => "".to_string()
            }
        )
    }

    pub fn serialize_no_sign(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            self.sender,
            self.receiver,
            self.amount,
            self.fee
        )
    }
}