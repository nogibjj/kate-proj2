# Project 2 - Recipe Randomizer
[![Rustfmt](https://github.com/nogibjj/kate-proj2/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/nogibjj/kate-proj2/actions/workflows/rustfmt.yml)
[![Clippy](https://github.com/nogibjj/kate-proj2/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/kate-proj2/actions/workflows/lint.yml)
[![Build binary release](https://github.com/nogibjj/kate-proj2/actions/workflows/release.yml/badge.svg)](https://github.com/nogibjj/kate-proj2/actions/workflows/release.yml)

My project 2 is a Rust Microservice that randomly generates recipes and helps users decide what to eat for dinner. This project is based on actix and a free food API called TheMealDB.

## Example
![/](0.png "/").
![/meal](1.png "/meal").
![/duke](2.png "/duke").

## How to use
* `"/"`: The root directory, displays the message "What to eat for dinner?"
* `"/meal"`: Randomly generates a recipe json. There are many details to the recipe, including: mealname, ingredients, measures of ingredients, instructions, an image, youtube tutorial, meal category, origin country, etc.
* `/duke`: If you are a Duke student, you can use this link to get a random dining place at Broadhead Center and Bryan Center.

## Future Work
* Deploy on Kubernetes

## References
* [TheMealDB](https://www.themealdb.com/)
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
