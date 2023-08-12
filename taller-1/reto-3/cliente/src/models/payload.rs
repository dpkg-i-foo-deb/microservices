use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct Payload {
    data:String
}

impl Payload{
    pub fn new (data:String) -> Payload {
        Payload { data }
    }

    pub fn data (&self) -> String{
        self.data.to_owned()
    }

    pub fn to_json(&self) -> Option<String> {
        let result = serde_json::to_string(self);

        match result {
            Ok(json)=>{Some(json)},
            Err(_)=>{None}
        }
    }
}