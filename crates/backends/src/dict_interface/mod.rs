use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct WordExplanation {
    pub(crate) word: String,
    pub(crate) phonetics: Vec<String>,
    pub(crate) explanations: Vec<Explanation>,
    pub(crate) idioms: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Explanation {
    pub(crate) phonetics: Option<Vec<String>>,
    pub(crate) explanation: String,
    pub(crate) definition: String,
    pub(crate) patterns: Option<Vec<String>>,
    pub(crate) sentences: Option<Vec<Sentence>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Sentence {
    pub(crate) sentence: String,
    pub(crate) translation: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Idiom {
    pub(crate) idiom: String,
    pub(crate) explanation: String,
    pub(crate) sentences: Option<Vec<Sentence>>,
}

// Example
pub(crate) fn example_arrive_word_explanation() -> WordExplanation {
    WordExplanation {
        word: "arrive".to_string(),
        phonetics: vec!["əˈraɪv".to_string()],
        explanations: vec![Explanation {
            phonetics: Some(vec!["əˈraɪv".to_string()]),
            explanation: "to reach a destination; to come to a place".to_string(),
            definition: "the act of arriving at a destination".to_string(),
            patterns: None,
            sentences: Some(vec![Sentence {
                sentence: "The train will arrive on time.".to_string(),
                translation: "火车将准时到达。".to_string(),
            }]),
        }],
        idioms: None,
    }
}
