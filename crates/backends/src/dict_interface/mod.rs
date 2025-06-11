use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WordExplanation {
    pub word: String,
    pub phonetics: Option<Vec<String>>,
    pub explanations: Option<Vec<Explanation>>,
    pub idioms: Option<Vec<Idiom>>,
    pub phrasal_verbs: Option<Vec<PhrasalVerb>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Explanation {
    pub phonetics: Option<Vec<String>>,
    pub abbreviation: Option<String>,
    pub explanation: String,
    pub definition: String,
    pub patterns: Option<Vec<String>>,
    pub examples: Option<Vec<Example>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Example {
    pub example: String,
    pub translation: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Idiom {
    pub idiom: String,
    pub explanation: String,
    pub definition: String,
    pub example: Option<Vec<Example>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PhrasalVerb {
    pub phrasal_verb: String,
    pub explanation: String,
    pub definition: String,
    pub example: Option<Vec<Example>>,
}

// Example
pub fn example_arrive_word_explanation() -> WordExplanation {
    WordExplanation {
        word: "arrive".to_string(),
        phonetics: Some(vec!["/əˈraɪv/".to_string()]),
        explanations: Some(vec![
            Explanation {
                abbreviation: Some("arr.".to_string()),
                phonetics: None,
                explanation: "到达，抵达".to_string(),
                definition: "to get to a place, especially at the end of a journey".to_string(),
                patterns: Some(vec!["~/(at/in/on ...)".to_string()]),
                examples: Some(vec![
                    Example {
                        example: "The train will arrive on time.".to_string(),
                        translation: "火车将准时到达。".to_string(),
                    },
                    Example {
                        example: "By the time I *arrived on the scene*, it was all over."
                            .to_string(),
                        translation: "我来到现场时，一切都结束了。".to_string(),
                    },
                ]),
            },
            Explanation {
                abbreviation: None,
                phonetics: None,
                explanation: "（东西）送达；寄到".to_string(),
                definition: "(of things) to be brought to sb".to_string(),
                patterns: None,
                examples: Some(vec![
                    Example {
                        example: "A letter arrived for you this morning".to_string(),
                        translation: "今天早上来了一封给你的信".to_string(),
                    },
                    Example {
                        example: "Send your application to arrive by 31 October".to_string(),
                        translation: "申请信要在 10 月 31 日前寄到".to_string(),
                    },
                ]),
            },
        ]),
        idioms: Some(vec![Idiom {
            idiom: "sb has arrived".to_string(),
            definition: "(informal) somebody has become successful".to_string(),
            explanation: "某人成功了".to_string(),
            example: Some(vec![Example {
                example: "He knew he had arrived when he waws shortlisted for the Booker prize"
                    .to_string(),
                translation: "被列入布克小说作品奖决选名单后，他知道自己成功了".to_string(),
            }]),
        }]),
        phrasal_verbs: Some(vec![PhrasalVerb {
            phrasal_verb: "arrive at sth".to_string(),
            explanation: "达成（协议）；作出（决议等）；得出（结论等）".to_string(),
            definition: "to decide on or find sth, expecially after discussion and thought"
                .to_string(),
            example: Some(vec![
                Example {
                    example: "to arrive at an agreement/a decision/a conclusion".to_string(),
                    translation: "达成协议；作出决定；得出结论".to_string(),
                },
                Example {
                    example: "to arrive at the truth".to_string(),
                    translation: "找到真理".to_string(),
                },
            ]),
        }]),
    }
}
