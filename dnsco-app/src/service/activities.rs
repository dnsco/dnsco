pub use templates::ListTemplate;

mod templates {
    use askama::Template;
    //    use chrono::prelude::*;
    //    use chrono::Duration;
    use url::Url;

    use crate::app::SiteUrls;
    use dnsco_data::models::Activity;

    #[derive(Template)]
    #[template(path = "activities.html")]
    pub struct ListTemplate {
        pub activities: ActivitiesCollection,
        pub update_url: Url,
    }

    // - {{activity.distance.miles()}}
    // - {{ activity.total_elevation_gain.feet()}} vert

    impl ListTemplate {
        pub fn new(activities: Vec<Activity>, urls: &SiteUrls) -> Self {
            Self {
                activities: ActivitiesCollection(activities),
                update_url: urls.update_activities(),
            }
        }
    }

    pub struct ActivitiesCollection(pub Vec<Activity>);

    impl ActivitiesCollection {
        pub fn last_seven_days(&self) -> Vec<&Activity> {
            //            let seven_days_ago = Utc::now() - Duration::days(7);
            self.0
                .iter()
                //                .filter(|a| a.start_date > seven_days_ago)
                .collect()
        }
    }

}
