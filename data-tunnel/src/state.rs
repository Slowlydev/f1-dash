use std::collections::HashMap;

use merge::Merge;
use serde_json::Value;

use crate::f1_socket::f1_models::{SocketData, A, R};

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
                let category: String = message.a.0;
                let message: A = serde_json::from_value::<A>(message.a.1).unwrap(); // TODO remove

                println!("{addition:?}");

                // let data_additon: R = serde_json::

                // self.value.merge(data_additon);
            }
        }

        if let Some(data) = data.r {
            println!("Got Replay data");
            self.value.merge(data);
        }
    }

    // fn decompress()
}
