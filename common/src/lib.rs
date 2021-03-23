use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Ping,
    Pong,
}

pub fn pack<T: Serialize>(data: &T) -> Vec<u8> {
    bincode::serialize(data).expect("Error serializing data")
}

pub fn unpack<T: DeserializeOwned>(data: &Vec<u8>) -> T {
    bincode::deserialize(data).expect("Error deserializing data")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
