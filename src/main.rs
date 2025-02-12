#[macro_use]
extern crate rocket;

use lazy_static::lazy_static;
use rand::Rng;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::serde::Serialize;

const YES_NO: [&str; 2] = ["yes", "no"];

lazy_static! {
    static ref ANSWERS: Arc<Mutex<HashMap<String, Answer>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref GIPHY_API_KEY: String = include_str!("giphy-api-key.txt").trim().to_string();
}

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
struct Answer {
    answer: &'static str,
    gif_url: String,
}

async fn create_new_answer() -> Answer {
    // Random yes/no answer
    let answer = {
        let mut rng = rand::rng();
        let index = rng.random_range(0..YES_NO.len());
        YES_NO[index]
    };

    // Get a gif from Giphy
    let url = format!(
        "https://api.giphy.com/v1/gifs/random?api_key={}&tag={}",
        *GIPHY_API_KEY, answer
    );
    let response = reqwest::get(&url)
        .await
        .unwrap()
        .json::<rocket::serde::json::Value>()
        .await
        .unwrap();
    let gif_url = response["data"]["images"]["original"]["url"]
        .as_str()
        .unwrap()
        .to_string();

    // Return 'em both
    Answer {
        answer: answer,
        gif_url: gif_url,
    }
}

async fn get_or_create_answer(key: &str) -> Answer {
    // Lock and check if the answer already exists
    {
        let answers = ANSWERS.lock().unwrap();
        if let Some(answer) = answers.get(key) {
            return answer.clone();
        }
    }

    // If none exists then release the lock (so we don't block
    // other threads while we do time consuming work, then
    // create a new answer and reaqure the lock to insert it
    let new_answer = create_new_answer().await;
    ANSWERS
        .lock()
        .unwrap()
        .insert(key.to_string(), new_answer.clone());
    new_answer
}

#[get("/api/<key>")]
async fn api_answer(key: &str) -> Json<Answer> {
    Json(get_or_create_answer(key).await)
}

#[get("/<key>")]
async fn html_answer(key: &str) -> RawHtml<String> {
    let answer = get_or_create_answer(key).await;
    let html = include_str!("answer.html")
        .replace("!!URL!!", &answer.gif_url)
        .replace("!!ANSWER!!", answer.answer)
        .to_string();
    RawHtml(html)
}

#[get("/")]
fn redirect_to_rand_answer() -> Redirect {
    let mut rng = rand::rng();
    let key = rng.random::<u64>().to_string();
    Redirect::to(format!("/{}", key))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![api_answer, html_answer, redirect_to_rand_answer],
    )
}
