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

table! {
    oauth_tokens (id) {
        id -> Int4,
        token -> Varchar,
        refresh -> Varchar,
        remote_athlete_id -> Int4,
        expires_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(activities, oauth_tokens,);
