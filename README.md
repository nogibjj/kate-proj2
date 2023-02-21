# Project 2 - Recipe Randomizer
[![Rustfmt](https://github.com/nogibjj/kate-proj2/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/nogibjj/kate-proj2/actions/workflows/rustfmt.yml)
[![Clippy](https://github.com/nogibjj/kate-proj2/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/kate-proj2/actions/workflows/lint.yml)
[![Build binary release](https://github.com/nogibjj/kate-proj2/actions/workflows/release.yml/badge.svg)](https://github.com/nogibjj/kate-proj2/actions/workflows/release.yml)

My project 2 is a Rust Microservice that randomly generates recipes and helps users decide what to eat for dinner. This project is based on actix and a free food API called TheMealDB.

## Feature
* `"/"`: The root directory, displays the message "What to eat for dinner?"
![/](0.png "/")

* `"/meal"`: Randomly generates a recipe json. There are many details to the recipe, including: mealname, ingredients, measures of ingredients, instructions, an image, youtube tutorial, meal category, origin country, etc.
![/meal](1.png "/meal")

* `/duke`: If you are a Duke student, you can use this link to get a random dining place at Broadhead Center and Bryan Center.
![/duke](2.png "/duke")

## How to use
1. pull the docker image from ECR: `docker pull public.ecr.aws/r9c5u1t7/recipe-randomizer`
2. run the docker image: `docker run -it --rm -p 8080:8080 public.ecr.aws/r9c5u1t7/recipe-randomizer`

I have also deployed it on APP RUNNER. Check it out: https://pjhimsgwbh.us-east-1.awsapprunner.com 


## Future Work
* Deploy on Kubernetes

## References
* [TheMealDB](https://www.themealdb.com/)
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
