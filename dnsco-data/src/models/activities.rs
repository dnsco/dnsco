use diesel::prelude::*;

use strava::models::activity::Summary as StravaActivity;

use crate::database::Connection;
use crate::schema::activities;
use crate::schema::activities::dsl::*;
use crate::{DataError, DataResult};

pub mod commands {
    use crate::{DataResult, RequestContext};

    pub fn update_from_strava(context: RequestContext) -> DataResult<()> {
        let token = context.tokens_repo().get().ok();
        let strava_api = context.strava_api().api(token)?;
        context
            .activities_repo()
            .batch_upsert_from_strava(strava_api.activities()?)?;
        Ok(())
    }
}

#[derive(Queryable)]
pub struct Activity {
    pub id: i32,
    pub description: Option<String>,
    pub distance: Option<f64>,
    pub name: String,
    pub remote_athlete_id: i32,
    pub remote_id: i32,
}

pub struct Repo<'a> {
    pub connection: &'a Connection,
}

impl<'a> Repo<'a> {
    pub fn all(&self) -> DataResult<Vec<Activity>> {
        activities
            .load(self.connection)
            .map_err(DataError::QueryError)
    }

    pub fn upsert(&self, activity: &NewActivity) -> DataResult<usize> {
        diesel::insert_into(activities::table)
            .values(activity)
            .on_conflict(remote_id)
            .do_update()
            .set(activity)
            .execute(self.connection)
            .map_err(DataError::QueryError)
    }

    pub fn batch_upsert_from_strava(&self, acts: Vec<StravaActivity>) -> DataResult<Vec<usize>> {
        //Todo N+1 lol and panic
        let all_work: Result<Vec<usize>, _> = acts
            .iter()
            .map(|a| {
                let x: NewActivity = a.into();
                self.upsert(&x)
            })
            .collect();

        all_work.map_err(DataError::from)
    }
}

#[derive(AsChangeset, Insertable)]
#[table_name = "activities"]
pub struct NewActivity<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub distance: Option<f64>,
    pub remote_athlete_id: i32,
    pub remote_id: i32,
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
