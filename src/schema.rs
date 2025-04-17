// @generated automatically by Diesel CLI.

diesel::table! {
    config (id) {
        id -> Nullable<Integer>,
        first_visit -> Bool,
    }
}

diesel::table! {
    diskinfo (id) {
        id -> Nullable<Integer>,
        sysinfo_id -> Integer,
        filesystem -> Text,
        size -> Integer,
        used -> Integer,
        available -> Integer,
        used_percentage -> Integer,
        mounted_path -> Text,
    }
}

diesel::table! {
    services (id) {
        id -> Nullable<Integer>,
        name -> Text,
        link -> Text,
        icon -> Nullable<Text>,
        user_id -> Integer,
    }
}

diesel::table! {
    sysinfo (id) {
        id -> Nullable<Integer>,
        timestamp -> Text,
        cpu_usage -> Float,
        ram_usage -> Float,
        total_ram -> Float,
        free_ram -> Float,
        used_swap -> Float,
    }
}

diesel::table! {
    user (id) {
        id -> Nullable<Integer>,
        username -> Text,
        password -> Text,
    }
}

diesel::joinable!(diskinfo -> sysinfo (sysinfo_id));
diesel::joinable!(services -> user (user_id));
// diesel::joinable!(sysinfo -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    config,
    diskinfo,
    services,
    sysinfo,
    user,
);
