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
    }
}
