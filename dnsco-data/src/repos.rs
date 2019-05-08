use crate::models::{Event, Race};

pub struct Events {}

impl Events {
    pub fn events(&self) -> Vec<Event> {
        vec![
            Event {
                name: "Marin Ultra Challenge",
                time: "2019-03-09",
                info: Race { distance: "25k " },
            },
            Event {
                name: "Behind the Rocks",
                time: "2019-03-23",
                info: Race { distance: "30k" },
            },
            Event {
                name: "Broken Arrow Skyrace",
                time: "2019-06-23",
                info: Race { distance: "26k " },
            },
        ]
    }
}
