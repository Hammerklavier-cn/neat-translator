use std::{
    rc::Rc,
    sync::{Arc, Mutex, mpsc},
};

use anyhow::{Error, Result};
use backends::{
    QwenWordSentenceTranslator, SentenceTranslator, StreamSentenceTranslator, WordTranslator,
    dict_interface::WordExplanation,
};
use slint::{Model, ModelRc, VecModel};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

slint::include_modules!();

pub fn run() -> Result<(), slint::PlatformError> {
    log::info!("Using slint interface");
    // First initialise and load api-keys, etc.
    let profile = match backends::initialise() {
        Ok(s) => s,
        Err(e) => {
            let error_window = ErrorWindow::new()?;

            error_window
                .as_weak()
                .upgrade_in_event_loop(move |window| {
                    window.set_error_hint("Initialisation Failed Error:".into());
                    window.set_error_text(e.to_string().into());
                })
                .unwrap();

            error_window.run()?;

            return Ok(());
        }
    };

    let main_window = MainWindow::new()?;
    let main_window_weak_arc = Arc::new(main_window.as_weak());

    let about_slint_window = AboutSlintWindow::new()?;
    let setting_window = SettingWindow::new()?;
    let setting_window_weak_arc = Arc::new(setting_window.as_weak());

    // Update setting profile
    let _ = setting_window_weak_arc
        .clone()
        .upgrade_in_event_loop(move |handle| {
            handle.set_settings_from_slint(Settings {
                deepseek_api_key: match profile.ai_accounts {
                    Some(ref accounts) => match &accounts.deepseek {
                        Some(deepseek) => &deepseek.api_key,
                        None => "",
                    },
                    None => "",
                }
                .into(),
                qwen_api_key: match profile.ai_accounts {
                    Some(ref accounts) => match &accounts.qwen {
                        Some(qwen) => &qwen.api_key,
                        None => "",
                    },
                    None => "",
                }
                .into(),
            });
            handle.invoke_sync_settings_from_property();
        });

    // Save settings from Slint
    // TODO

    // Translate word
    //
    // Create a global Arc<Mutex<Receiver<WordExplanation>>> pointer.
    // The Receiver<WordExplanation>> will be replaced by a new one every time
    // the user sends a new callback to translate.
    let (_, rx) = mpsc::channel::<Result<WordExplanation, Error>>();
    let wd_rx_arc_mutex = Arc::new(Mutex::new(rx));
    std::thread::spawn({
        let main_window_weak_arc = main_window_weak_arc.clone();
        let wd_rx_arc_mutex = wd_rx_arc_mutex.clone();
        move || {
            loop {
                if let Some(received_result) = match wd_rx_arc_mutex.try_lock() {
                    Ok(rx) => {
                        log::trace!("Successfully acquired lock of WordExplanation.");
                        rx.try_recv().ok()
                    }
                    Err(_) => {
                        log::trace!("Failed to acquire lock of WordExplanation.");
                        None
                    }
                } {
                    match received_result {
                        Ok(received_we) => {
                            let _ = main_window_weak_arc.upgrade_in_event_loop(move |handle| {
                                let mut results: Vec<WordTransResult> = Vec::new();
                                results.push(WordTransResult {
                                    index: "".into(),
                                    text: "WORD".into(),
                                    type_: WordTransType::Header,
                                });
                                results.push(WordTransResult {
                                    index: "".into(),
                                    text: received_we.word.into(),
                                    type_: WordTransType::Word,
                                });
                                if let Some(phonetics) = received_we.phonetics {
                                    results.push(WordTransResult {
                                        index: "".into(),
                                        text: phonetics.join(", ").into(),
                                        type_: WordTransType::Phonetic,
                                    });
                                }
                                results.push(WordTransResult {
                                    index: "".into(),
                                    text: "EXPLANATION".into(),
                                    type_: WordTransType::Header,
                                });
                                let mut index = 0;

                                for part_of_speech in
                                    received_we.part_of_speeches.unwrap_or_default()
                                {
                                    results.push(WordTransResult {
                                        index: "".into(),
                                        text: part_of_speech.part_of_speech.to_string().into(),
                                        type_: WordTransType::PartOfSpeech,
                                    });
                                    for explanation in part_of_speech.explanations {
                                        index += 1;
                                        let mut text = String::new();
                                        if let Some(phonetics) = explanation.phonetics {
                                            text.push_str(&format!("{} ", phonetics.join(", ")));
                                        }
                                        if let Some(abbr) = explanation.abbreviation {
                                            text.push_str(&format!("(abbr. {}) ", abbr));
                                        }
                                        if let Some(patterns) = explanation.patterns {
                                            text.push_str(&format!("({})", patterns.join(", ")));
                                        }
                                        text.push_str(&format!("{} ", explanation.definition));
                                        results.push(WordTransResult {
                                            index: index.to_string().into(),
                                            text: text.into(),
                                            type_: WordTransType::Explanation,
                                        });
                                        results.push(WordTransResult {
                                            index: "".into(),
                                            text: explanation.explanation.into(),
                                            type_: WordTransType::Explanation,
                                        });
                                        if let Some(examples) = explanation.examples {
                                            for example in examples {
                                                results.push(WordTransResult {
                                                    index: "".into(),
                                                    text: example.example.into(),
                                                    type_: WordTransType::Example,
                                                });
                                                results.push(WordTransResult {
                                                    index: "".into(),
                                                    text: example.translation.into(),
                                                    type_: WordTransType::ExampleTranslation,
                                                });
                                            }
                                        }
                                    }

                                    if let Some(idioms) = part_of_speech.idioms {
                                        results.push(WordTransResult {
                                            index: "".into(),
                                            text: "IDIOMS".into(),
                                            type_: WordTransType::Header,
                                        });

                                        let mut index = 0;

                                        for idiom in idioms {
                                            index += 1;
                                            results.push(WordTransResult {
                                                index: index.to_string().into(),
                                                text: idiom.idiom.into(),
                                                type_: WordTransType::IdiomAndPhrase,
                                            });
                                            results.push(WordTransResult {
                                                index: "".into(),
                                                text: idiom.explanation.into(),
                                                type_: WordTransType::Explanation,
                                            });
                                            results.push(WordTransResult {
                                                index: "".into(),
                                                text: idiom.definition.into(),
                                                type_: WordTransType::Definition,
                                            });
                                            for example in idiom.example.unwrap_or_default() {
                                                results.push(WordTransResult {
                                                    index: "".into(),
                                                    text: example.example.into(),
                                                    type_: WordTransType::Example,
                                                });
                                                results.push(WordTransResult {
                                                    index: "".into(),
                                                    text: example.translation.into(),
                                                    type_: WordTransType::ExampleTranslation,
                                                });
                                            }
                                        }
                                    }

                                    if let Some(phrasal_verbs) = part_of_speech.phrasal_verbs {
                                        results.push(WordTransResult {
                                            index: "".into(),
                                            text: "PHRASAL VERBS".into(),
                                            type_: WordTransType::Header,
                                        });

                                        let mut index = 0;

                                        for phrasal_verb in phrasal_verbs {
                                            index += 1;
                                            results.push(WordTransResult {
                                                index: index.to_string().into(),
                                                text: phrasal_verb.phrasal_verb.into(),
                                                type_: WordTransType::IdiomAndPhrase,
                                            });
                                            results.push(WordTransResult {
                                                index: "".into(),
                                                text: phrasal_verb.explanation.into(),
                                                type_: WordTransType::Explanation,
                                            });
                                            results.push(WordTransResult {
                                                index: "".into(),
                                                text: phrasal_verb.definition.into(),
                                                type_: WordTransType::Definition,
                                            });
                                            for example in phrasal_verb.example.unwrap_or_default()
                                            {
                                                results.push(WordTransResult {
                                                    index: "".into(),
                                                    text: example.example.into(),
                                                    type_: WordTransType::Example,
                                                });
                                                results.push(WordTransResult {
                                                    index: "".into(),
                                                    text: example.translation.into(),
                                                    type_: WordTransType::ExampleTranslation,
                                                });
                                            }
                                        }
                                    }
                                }

                                let vec_model_results =
                                    ModelRc::from(Rc::new(VecModel::from(results)));
                                handle.set_word_trans_results(ModelRc::from(vec_model_results));
                            });
                        }
                        Err(e) => {
                            log::warn!("Error translating word: {:#}", e);
                            let _ =
                                main_window_weak_arc.upgrade_in_event_loop(move |main_window| {
                                    main_window.set_word_trans_results(ModelRc::from(Rc::new(
                                        VecModel::from(vec![WordTransResult {
                                            index: "".into(),
                                            text: e.to_string().into(),
                                            type_: WordTransType::Error,
                                        }]),
                                    )));
                                });
                        }
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    });
    //
    main_window.global::<Logic>().on_translate_word({
        let main_window_weak_arc = main_window_weak_arc.clone();

        let wd_rx_arc_mutex = wd_rx_arc_mutex.clone();

        let setting_window_weak_arc = setting_window_weak_arc.clone();

        move |text| {
            // Implement translation logic here
            log::info!("Translate Word: {}", text.to_uppercase());

            let setting_window = setting_window_weak_arc.clone().upgrade().unwrap();
            let settings_from_slint = setting_window.get_settings_from_slint();

            let api_key = settings_from_slint.qwen_api_key.to_string();
            log::info!("Got api_key from settings_from_slint: {}", api_key);
            let translator = Box::new(QwenWordSentenceTranslator::new(api_key))
                as Box<dyn WordTranslator + Send + Sync>;

            let (tx, rx) = mpsc::channel();
            *wd_rx_arc_mutex.lock().unwrap() = rx;

            std::thread::spawn(move || {
                let result = translator.translate_word(
                    &text.to_string(),
                    backends::Language::English,
                    backends::Language::Chinese,
                );
                if let Err(e) = tx.send(result) {
                    log::info!(
                        "Error sending message, maybe because Receiver is dropped: {}",
                        e
                    );
                }
            });
        }
    });

    // Show about slint
    main_window.on_show_about_slint(move || {
        // Implement show about slint logic here
        log::info!("Show About Slint window");
        about_slint_window.show().unwrap();
    });

    // Show setting window
    main_window.on_show_setting_window({
        let setting_window = setting_window_weak_arc.clone().upgrade().unwrap();
        move || {
            log::info!("Show Setting Window");
            setting_window.show().unwrap();
        }
    });

    // Save settings
    setting_window.global::<Logic>().on_save_settings({
        let setting_window_weak_arc = setting_window_weak_arc.clone();
        move || {
            log::trace!("Save Settings");
            let settings_from_slint = setting_window_weak_arc
                .clone()
                .upgrade()
                .unwrap()
                .get_settings_from_slint();

            let setting: backends::storage::Settings = backends::storage::Settings {
                ai_accounts: {
                    let deepseek_api_key = settings_from_slint.deepseek_api_key.to_string();
                    let qwen_api_key = settings_from_slint.qwen_api_key.to_string();
                    if deepseek_api_key.is_empty() && qwen_api_key.is_empty() {
                        None
                    } else {
                        Some(backends::storage::AiAccounts {
                            deepseek: {
                                if !deepseek_api_key.is_empty() {
                                    Some(backends::storage::DeepSeek {
                                        api_key: deepseek_api_key,
                                    })
                                } else {
                                    None
                                }
                            },
                            qwen: {
                                if !qwen_api_key.is_empty() {
                                    Some(backends::storage::Qwen {
                                        api_key: qwen_api_key,
                                    })
                                } else {
                                    None
                                }
                            },
                        })
                    }
                },
                behaviour: None,
                appearance: None,
            };

            // write to disk
            if let Err(e) = backends::save_config(&setting) {
                log::error!("Failed to save config: {}", e);

                let error_window = ErrorWindow::new().unwrap();

                error_window
                    .as_weak()
                    .upgrade_in_event_loop(move |window| {
                        window.set_error_hint("Save config Error:".into());
                        window.set_error_text(e.to_string().into());
                    })
                    .unwrap();

                error_window.run().unwrap();
            }
        }
    });

    // Translate sentence
    //
    // Create a global Arc<Mutex<Receiver<String>>> pointer.
    // The Receiver<String>> will be replaced with a new one every time
    // the user sends a new callback.
    let (_, rx) = mpsc::channel::<String>();
    let st_rx_arc_mutex = Arc::new(Mutex::new(rx));
    std::thread::spawn({
        let main_window_weak_arc = main_window_weak_arc.clone();
        let st_rx_arc_mutex = st_rx_arc_mutex.clone();
        move || {
            let mut received_flag: bool;
            loop {
                if let Some(received_string) = match st_rx_arc_mutex.try_lock() {
                    Ok(rx) => {
                        log::trace!("Successfully acquired lock");
                        rx.try_recv().ok()
                    }
                    Err(_) => {
                        log::trace!("Failed to acquire lock!");
                        None
                    } // The mutex lock is dropped here
                } {
                    received_flag = true;
                    let _ = main_window_weak_arc.upgrade_in_event_loop(move |handle| {
                        // Update the UI with the received translation result immediately and swiftly
                        // to avoid blocking the main thread.
                        handle.set_sentence_translate_result(received_string.into());
                    });
                } else {
                    received_flag = false;
                }
                if !received_flag {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        }
    });
    // Logic implementation
    main_window.global::<Logic>().on_translate_sentence({
        let main_window_weak_arc = main_window_weak_arc.clone();
        let setting_window_weak_arc = setting_window_weak_arc.clone();

        let rx_arc_mutex = st_rx_arc_mutex.clone();

        move |text, from_language, to_language, model| {
            let main_window_weak = main_window_weak_arc.clone();
            let main_window = main_window_weak.upgrade().unwrap();

            // let api_key = main_window.get_api_key().to_string();
            let setting_window = setting_window_weak_arc.clone().upgrade().unwrap();
            let settings_from_slint = setting_window.get_settings_from_slint();

            let text = text.to_string();
            let from_language = from_language.to_string().to_lowercase();
            let to_language = to_language.to_string().to_lowercase();
            let model = model.to_string().to_lowercase();
            log::trace!(
                "Translate {} from {} to {} with {}",
                text,
                from_language,
                to_language,
                model
            );

            let from_language = match from_language.as_str() {
                "chinese" => backends::Language::Chinese,
                "english" => backends::Language::English,
                "french" => backends::Language::French,
                "german" => backends::Language::German,
                "russian" => backends::Language::Russian,
                "japanese" => backends::Language::Japanese,
                "korean" => backends::Language::Korean,
                "spanish" => backends::Language::Spanish,
                _ => {
                    log::error!("Unsupported language: {}", from_language);
                    let (tx, rx) = mpsc::channel();
                    *rx_arc_mutex.lock().unwrap() = rx;
                    tx.send(format!("Error: Unsupported language: {}", from_language))
                        .unwrap();
                    return;
                }
            };

            let to_language = match to_language.as_str() {
                "chinese" => backends::Language::Chinese,
                "english" => backends::Language::English,
                "french" => backends::Language::French,
                "german" => backends::Language::German,
                "russian" => backends::Language::Russian,
                "japanese" => backends::Language::Japanese,
                "korean" => backends::Language::Korean,
                "spanish" => backends::Language::Spanish,
                _ => {
                    log::error!("Unsupported language: {}", from_language);
                    let (tx, rx) = mpsc::channel();
                    *rx_arc_mutex.lock().unwrap() = rx;
                    tx.send(format!("Error: Unsupported language: {}", from_language))
                        .unwrap();
                    return;
                }
            };

            let translator: Box<dyn StreamSentenceTranslator + Send + Sync> = match model.as_str() {
                "deepseek" => {
                    let api_key = settings_from_slint.deepseek_api_key.to_string();
                    log::info!("Got api_key from settings_from_slint: {}", api_key);
                    Box::new(backends::DeepSeekSentenceTranslator::new(api_key))
                }
                "youdao" => {
                    let (tx, rx) = mpsc::channel();
                    *rx_arc_mutex.lock().unwrap() = rx;
                    tx.send("Youdao api is not supported yet!!".into()).unwrap();
                    return;
                }
                "qwen" => {
                    let api_key = settings_from_slint.qwen_api_key.to_string();
                    log::info!("Got api_key from settings_from_slint: {}", api_key);
                    let (tx, rx) = mpsc::channel();
                    *rx_arc_mutex.lock().unwrap() = rx;
                    tx.send("Qwen api is not supported yet!!".into()).unwrap();
                    return;
                }
                _ => {
                    let (tx, rx) = mpsc::channel();
                    *rx_arc_mutex.lock().unwrap() = rx;
                    tx.send("Unknown AI api".into()).unwrap();
                    return;
                }
            };

            if text == String::new() {
                log::debug!("Detect empty string, skip translating.");
                let (tx, rx) = mpsc::channel();
                *rx_arc_mutex.lock().unwrap() = rx;
                tx.send("[empty]".into()).unwrap();
                return;
            }

            // update translation result with a spawned thread to avoid blocking the UI.
            std::thread::spawn({
                let rx_arc_mutex = rx_arc_mutex.clone();
                move || {
                    let translate_result_rx = translator
                        .stream_translate_sentence(&text, from_language, to_language)
                        .unwrap();

                    *rx_arc_mutex.lock().unwrap() = translate_result_rx;
                }
            });
        }
    });

    main_window.run()
}
