# Project 2 - Recipe Randomizer
[![Rustfmt](https://github.com/nogibjj/kate-proj2/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/nogibjj/kate-proj2/actions/workflows/rustfmt.yml)
[![Clippy](https://github.com/nogibjj/kate-proj2/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/kate-proj2/actions/workflows/lint.yml)
[![Build binary release](https://github.com/nogibjj/kate-proj2/actions/workflows/release.yml/badge.svg)](https://github.com/nogibjj/kate-proj2/actions/workflows/release.yml)

My project 2 is a Rust Microservice that randomly generates recipes and helps users decide what to eat for dinner. This project is based on actix and a free food API called TheMealDB.

## Example


## How to use
* `"/"`: the root directory, displays the message "What to eat for dinner?"
* `"/meal"`: randomly generates a recipe json. There are many details to the recipe, including: mealname, ingredients, measures of ingredients, instructions, an image, youtube tutorial, meal category, origin country, etc.

## Future Work
* Add more options and links
* Kubernetes

## References
* [TheMealDB](https://www.themealdb.com/)
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
