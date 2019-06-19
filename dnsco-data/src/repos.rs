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
pub mod activities_repo {
    use diesel::pg::upsert::*;
    use diesel::prelude::*;

    use crate::database::Connection;
    use crate::models::{Activity, NewActivity};
    use crate::schema::activities;
    use crate::schema::activities::dsl::*;

    use strava::models::activity::Summary as StravaActivity;

    pub struct Repo<'a> {
        pub connection: &'a Connection,
    }

    impl<'a> Repo<'a> {
        pub fn all(&self) -> Vec<Activity> {
            activities.load(self.connection).expect("plz")
        }

        pub fn upsert(&self, activity: &NewActivity) -> diesel::QueryResult<usize> {
            dbg!(diesel::query_builder::AsChangeset::as_changeset(activity));
            dbg!(name.eq(excluded(name)));
            diesel::insert_into(activities::table)
                .values(activity)
                .on_conflict(remote_id)
                .do_update()
                .set(activity)
                .execute(self.connection)
        }

        pub fn batch_upsert_from_strava(&self, acts: Vec<StravaActivity>) {
            //Todo N+1 lol
            acts.iter().for_each(|a| {
                let x: NewActivity = a.into();
                self.upsert(&x).unwrap();
            })
        }
    }

    impl<'a> From<&'a StravaActivity> for NewActivity<'a> {
        fn from(act: &StravaActivity) -> NewActivity {
            NewActivity {
                name: &act.name,
                description: None, //Todo deal with nullstrings
                distance: None,    //Todo same
                remote_athlete_id: act.athlete.id as i32,
                remote_id: act.id as i32,
            }
        }
    }

}
