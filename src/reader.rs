use serde::Deserialize;
use serde_json::Result;
use std::{fs::File, io::BufReader};

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct Question {
    pub question: String,
    pub answers: Vec<String>,
    pub correct_answer: usize,
}
pub fn read_json() -> Result<Vec<Question>> {
    let file = File::open("quiz.json").unwrap_or_else(|_| {
        panic!("Can't find file 'quiz.json'");
    });
    let reader = BufReader::new(file);

    // Parse the string of data into serde_json::Value.
    let v: Vec<Question> = serde_json::from_reader(reader)?;

    Ok(v)
}
