#[macro_use]
extern crate rocket;

use backend::models::*;
use diesel::QueryDsl;
use rand::prelude::*;
use rocket::fs::FileServer;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket_sync_db_pools::{database, diesel, diesel::RunQueryDsl};
use std::env;

use backend::schema::*;

#[database("sqlite_locs")]
struct LocsDbConn(diesel::SqliteConnection);

#[get("/recent.json")]
async fn recent(
    conn: LocsDbConn,
    jar: &CookieJar<'_>,
) -> Result<Json<Vec<Geolocation>>, rocket::response::Debug<diesel::result::Error>> {
    let ck = match jar.get("submit_cookie") {
        Some(x) => Some(String::from(x.value())),
        None => None,
    };

    use backend::schema::geolocations::dsl::*;
    use diesel::ExpressionMethods;
    let x: Vec<Geolocation> = conn
        .run(move |c| {
            geolocations
                .filter(provisional.eq(Some(false)))
                .or_filter(cookie.eq(ck))
                .limit(500)
                .select(crate::geolocations::all_columns)
                .load(c)
        })
        .await?;

    Ok(Json(x))
}

#[get("/provisional.json")]
async fn provisional(
    conn: LocsDbConn,
) -> Result<Json<Vec<Geolocation>>, rocket::response::Debug<diesel::result::Error>> {
    use backend::schema::geolocations::dsl::*;
    use diesel::ExpressionMethods;
    let x: Vec<Geolocation> = conn
        .run(move |c| {
            geolocations
                .filter(provisional.eq(Some(true)))
                .or_filter(provisional.is_null())
                .limit(500)
                .select(crate::geolocations::all_columns)
                .load(c)
        })
        .await?;

    Ok(Json(x))
}

#[post("/location", format = "json", data = "<loc>")]
async fn addloc(
    loc: Json<NewGeolocation>,
    conn: LocsDbConn,
    jar: &CookieJar<'_>,
) -> Result<String, rocket::response::Debug<diesel::result::Error>> {
    let ck = match jar.get("submit_cookie") {
        Some(x) => Some(String::from(x.value())),
        None => None,
    };

    if ck.is_none() {
        let mut rng = rand::rngs::StdRng::from_entropy();

        let gns: [u8; 16] = rng.gen();

        let ck = base64::encode(gns);

        jar.add(Cookie::new("submit_cookie", ck));
    }

    let mut newloc = loc.clone().0;

    newloc.cookie = ck;

    newloc.uuid = Some(uuid::Uuid::new_v4().as_bytes().to_vec());
    conn.run(move |c| {
        diesel::insert_into(geolocations::table)
            .values(&newloc)
            .execute(c)
            .expect("whatever")
    })
    .await;

    Ok(String::from("OK"))
}

#[launch]
fn rocket() -> _ {
    println!("server started!");
    if env::var("PROD").is_ok() {
        rocket::build()
            .mount("/", FileServer::from("./frontend/dist/"))
            .mount("/api/", routes![recent, addloc, provisional])
            .attach(LocsDbConn::fairing())
    } else {
        rocket::build()
            .mount("/", routes![recent, addloc, provisional])
            .attach(LocsDbConn::fairing())
    }
}
