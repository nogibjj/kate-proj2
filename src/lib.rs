// use std::collections::HashMap;

pub async fn make_reqwest() -> serde_json::Value {
    let response = reqwest::get("https://www.themealdb.com/api/json/v1/1/random.php")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let response: serde_json::Value = serde_json::from_str(&response).unwrap();
    // let mut food = HashMap::new();
    // food.insert("mealname".to_string(), response["meals"][0]["strMeal"].as_str().unwrap().to_string());
    // food.insert("instructions".to_string(), response["meals"][0]["strInstructions"].as_str().unwrap().to_string());
    // food
    response
}

use rand::Rng;

pub const DUKE_DINING: [&str; 16] = [
    "Ginger & Soy",
    "Gyotaku",
    "Beyu Cafe",
    "Cafe",
    "Farmstead",
    "Il Forno",
    "JB's Roasts and Chops",
    "McDonald's",
    "Panda Express",
    "Panera",
    "Sazon",
    "Sprout",
    "Tandoor",
    "The Devil's Krafthouse",
    "The Loop",
    "The Skillet",
];

//create a function that returns a random fruit
pub fn random_dinning() -> &'static str {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..DUKE_DINING.len());
    DUKE_DINING[random_index]
}
