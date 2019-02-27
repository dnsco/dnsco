//use tokio::prelude::*;
use reqwest::header::AUTHORIZATION;

#[derive(Clone, Debug)]
pub struct Webserver {
    events: Vec<Event>,
    strava_token: String,
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
    pub fn new(strava_token: String) -> Self {
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
            strava_token,
        }
    }

    pub fn hello_world(&self) -> Result<IndexResponse, ()> {
        Ok(IndexResponse {
            events: self.events.to_vec(),
        })
    }

    pub fn activities(&self) -> Result<serde_json::Value, ()> {
        reqwest::Client::new()
            .get("https://www.strava.com/api/v3/athlete/activities")
            .header(AUTHORIZATION, format!("Bearer {}", self.strava_token))
            .send()
            .map_err(|_| ())?
            .json()
            .map_err(|_| ())
    }
}
