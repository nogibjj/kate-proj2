
use std::collections::HashMap;


// function to make reqwest to free dictionary api
pub async fn make_reqwest() -> HashMap<String, String> {
    let response = reqwest::get("https://www.themealdb.com/api/json/v1/1/random.php")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

        let mut food = HashMap::new();
        let response: serde_json::Value = serde_json::from_str(&response).unwrap();
        food.insert("mealname".to_string(), response["meals"][0]["strMeal"].as_str().unwrap().to_string());
        food
}
