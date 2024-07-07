## CLI Quiz

A command-line quiz game written in Rust.

## Features

* Reads questions and answers from a JSON file
* Asks the user to answer each question
* Keeps track of correct and incorrect answers
* Displays the results at the end

## Usage

1. Clone the repository: `git clone <repository_url>`
2. Build the project: `cargo build`
3. Run the quiz: `cargo run`

## JSON File Format

The quiz questions and answers are stored in a JSON file named `quiz.json`. The format is as follows:
```json
[
  {
    "question": "What is the capital of France?",
    "answers": ["Paris", "London", "Berlin"],
    "correct_answer": 0
  },
  {
    "question": "What is the largest planet in our solar system?",
    "answers": ["Earth", "Saturn", "Jupiter"],
    "correct_answer": 2
  }
]
