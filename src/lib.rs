// use std::collections::HashMap;

pub async fn make_reqwest() -> serde_json::Value{
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
