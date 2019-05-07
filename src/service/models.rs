use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Event {
    pub name: &'static str,
    pub time: &'static str,
    pub info: Race,
}

#[derive(Serialize, Clone, Debug)]
pub struct Race {
    pub distance: &'static str,
}
