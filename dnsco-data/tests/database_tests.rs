use std::time::UNIX_EPOCH;

use dnsco_data::models::activities::{NewActivity, Repo};
use dnsco_data::Database;

#[test]
fn test_db() {
    let db = Database::create("postgres://dennis@localhost/dnsco".to_owned());
    let connection = db.get_connection();

    let repo = Repo {
        connection: &connection,
    };

    let new_id = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time moves forward")
        .as_secs();

    let orig_count = repo.all().unwrap().len();
    let mut activity = NewActivity {
        name: "Hey",
        description: None,
        distance: Some(9.4),
        remote_athlete_id: 0,
        remote_id: new_id as i32,
    };

    repo.upsert(&activity).unwrap();
    assert_eq!(orig_count + 1, repo.all().unwrap().len());
    activity.description = Some("WHAT");
    repo.upsert(&activity).unwrap();
    let acts = repo.all().unwrap();
    let new_description = acts
        .iter()
        .find(|a| a.remote_id == 0)
        .unwrap()
        .description
        .as_ref()
        .unwrap();

    assert_eq!("WHAT", new_description);
}
