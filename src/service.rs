#[derive(Clone, Debug)]
pub struct Webserver {
    events: Vec<Event>,
}

#[derive(Serialize, Clone, Debug)]
struct Event {
    name: &'static str,
    time: &'static str,
    info: EventInfo,
}

#[derive(Serialize, Clone, Debug)]
enum EventInfo {
    Race { distance: &'static str }
}

#[derive(Serialize, Debug)]
pub struct IndexResponse {
    events: Vec<Event>
}

impl Webserver {
    pub fn new() -> Self {
        Self {
            events: vec![
                Event { name: "Marin Ultra Challenge", time: "2019-03-09", info: EventInfo::Race { distance: "25k " } },
                Event { name: "Behind the Rocks", time: "2019-03-23", info: EventInfo::Race { distance: "30k" } },
                Event { name: "Broken Arrow Skyrace", time: "2019-06-23", info: EventInfo::Race { distance: "26k " } },
            ],
        }
    }

    pub fn hello_world(&self) -> Result<IndexResponse, ()> {
        Ok(IndexResponse { events: self.events.to_vec() })
    }
}
