#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Serialize};
use rocket_db_pools::sqlx;
use rocket_db_pools::{Connection, Database};

pub mod blackjack;

#[derive(Database)]
#[database("postgres_score")]
struct ScoringDB(sqlx::PgPool);

#[allow(dead_code)]
struct ScoringRow {
    player: String,
    score: i64,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ScoringJSON {
    player_1: i64,
    player_2: i64,
}

#[get("/start")]
async fn index(mut db: Connection<ScoringDB>) -> &'static str {
    let res = blackjack::play();
    match res {
        "Player 1" => {
            sqlx::query!("update scoring set score = score+1 where player = 'Player1'")
                .execute(&mut *db)
                .await
                .expect("Could not update player score in DB");
            "Player 1 wins!"
        },
        "Player 2" => {
            sqlx::query!("update scoring set score = score+1 where player = 'Player2'")
                .execute(&mut *db)
                .await
                .expect("Could not update player score in DB");
            "Player 2 wins!"
        },
        _ => "It's a draw!",
    }
}

#[get("/scores")]
async fn scores_db(mut db: Connection<ScoringDB>) -> Json<ScoringJSON> {
    let score: Vec<ScoringRow> = sqlx::query_as!(ScoringRow, "select * from scoring")
        .fetch_all(&mut *db)
        .await
        .expect("Unable to query scoring table");
    Json(ScoringJSON {
        player_1: score[0].score,
        player_2: score[1].score,
    })
}

#[post("/reset-scores")]
async fn reset_scores(mut db: Connection<ScoringDB>) -> &'static str {
    sqlx::query!("update scoring set score = 0 where player = 'Player1'")
        .execute(&mut *db)
        .await
        .expect("Failed to reset Player 1's score");
    sqlx::query!("update scoring set score = 0 where player = 'Player2'")
        .execute(&mut *db)
        .await
        .expect("Failed to reset Player 2's score");

    "Scores reset to zero"
}

#[get("/")]
fn root() -> &'static str {
    "Welcome to the Blackjack API! Use /start to play a game and /scores to view scores."
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, scores_db, root, reset_scores])
        .attach(ScoringDB::init())
}
