// @generated automatically by Diesel CLI.

diesel::table! {
    geolocations (id) {
        id -> Integer,
        description -> Nullable<Text>,
        lat -> Double,
        lon -> Double,
        url -> Text,
        uuid -> Binary,
        notes -> Nullable<Text>,
        geolocator -> Nullable<Text>,
        datetime -> Nullable<Timestamp>,
        provisional -> Nullable<Bool>,
        cookie -> Nullable<Text>,
    }
}
