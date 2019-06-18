use askama::Template;
use chrono::prelude::*;
use chrono::Duration;

use strava::models::activity;

pub fn new(activities: Vec<activity::Summary>) -> Activities {
    Activities {
        activities: ActivitiesCollection(activities),
    }
}

#[derive(Template)]
#[template(path = "activities.html")]
pub struct Activities {
    pub activities: ActivitiesCollection,
}

type Activity = activity::Summary;

pub struct ActivitiesCollection(pub Vec<Activity>);

impl ActivitiesCollection {
    pub fn last_seven_days(&self) -> Vec<&Activity> {
        let seven_days_ago = Utc::now() - Duration::days(7);

        self.0
            .iter()
            .filter(|a| a.start_date > seven_days_ago)
            .collect()
    }
}
