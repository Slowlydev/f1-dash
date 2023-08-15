use merge::Merge;

use crate::f1_socket::f1_models::{SocketData, R};

pub struct State {
    pub value: R,
}

impl State {
    // pub fn translate() -> Value {}

    pub fn new() -> State {
        State {
            value: R::default(),
        }
    }

    pub fn update(&mut self, data: SocketData) {
        if let Some(data) = data.m {
            for message in data {
                if message.m != "feed" {
                    continue;
                };

                // get category and message
                let category: &String = &message.a.0;
                let message: &serde_json::Value = &message.a.1;

                println!("category: {category:?}");
                println!("message: {message:?}");
            }
        }

        if let Some(data) = data.r {
            println!("Got Replay data");
            self.value.merge(data);
        }
    }

    // fn decompress()
}
