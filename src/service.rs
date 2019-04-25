use crate::strava;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Webserver {
    events: Vec<Event>,
    strava_api: Arc<strava::Api>,
}

#[derive(Serialize, Clone, Debug)]
struct Event {
    name: &'static str,
    time: &'static str,
    info: Race,
}

#[derive(Serialize, Clone, Debug)]
struct Race {
    distance: &'static str,
}

#[derive(Serialize, Debug)]
pub struct IndexResponse {
    events: Vec<Event>,
}

impl Webserver {
    pub fn new(strava_api: Arc<strava::Api>) -> Self {
        Self {
            events: vec![
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
            ],
            strava_api,
        }
    }

    pub fn hello_world(&self) -> Result<IndexResponse, ()> {
        Ok(IndexResponse {
            events: self.events.to_vec(),
        })
    }

    pub fn activities(&self) -> Result<String, &'static str> {
        self.strava_api.activities().and_then(|activities| {
            serde_json::to_string(&activities).map_err(|_| "failed to re_serialize")
        })
    }
}
