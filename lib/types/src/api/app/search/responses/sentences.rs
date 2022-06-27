use serde::{Deserialize, Serialize};

/// Names API response. Contains all Names
#[derive(Serialize, Deserialize, Clone)]
pub struct Response {
    sentences: Vec<Sentence>,
}

impl Response {
    #[inline]
    pub fn new(sentences: Vec<Sentence>) -> Self {
        Self { sentences }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Sentence {
    content: String,
    translation: String,
}

impl Sentence {
    /// Create a new sentence
    #[inline]
    pub fn new(content: String, translation: String) -> Self {
        Self {
            content,
            translation,
        }
    }
}
