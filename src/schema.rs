table! {
    user_profile (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        nickname -> Varchar,
        gender -> Tinyint,
        birthday -> Nullable<Datetime>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    users (id) {
        id -> Unsigned<Integer>,
        email -> Nullable<Varchar>,
        hash -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

joinable!(user_profile -> users (user_id));
allow_tables_to_appear_in_same_query!(user_profile, users,);
