table! {
    activities (id) {
        id -> Int4,
        description -> Nullable<Text>,
        distance -> Nullable<Float8>,
        name -> Varchar,
        remote_athlete_id -> Int4,
        remote_id -> Int4,
    }
}
