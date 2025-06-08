use std::sync::{Arc, Mutex, mpsc};

use backends::{SentenceTranslator, StreamSentenceTranslator};

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
    main_window.global::<Logic>().on_translate_word(|text| {
        // Implement translation logic here
        println!("Translate Word: {}", text.to_uppercase());
        text.to_uppercase().into()
    });

    // Show about slint
    main_window.global::<Logic>().on_show_about_slint(move || {
        // Implement show about slint logic here
        println!("About Slint");
        about_slint_window.show().unwrap();
    });

    // Show setting window
    main_window.global::<Logic>().on_show_setting_window({
        let setting_window = setting_window_weak_arc.clone().upgrade().unwrap();
        move || {
            println!("Show Setting Window");
            setting_window.show().unwrap();
        }
    });

    // Save settings
    setting_window.global::<Logic>().on_save_settings({
        let setting_window_weak_arc = setting_window_weak_arc.clone();
        move || {
            println!("Save Settings");
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
                eprintln!("Failed to save config: {}", e);

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
    // Note that the logic in StreamTranslator::stream_translate_sentence
    // should be modified. It should handle the case when the rx is closed early.
    let (_, rx) = mpsc::channel::<String>();
    let rx_arc_mutex = Arc::new(Mutex::new(rx));
    std::thread::spawn({
        let main_window_weak_arc = main_window_weak_arc.clone();
        let rx_arc_mutex = rx_arc_mutex.clone();
        move || {
            let mut received_flag: bool;
            loop {
                if let Some(received_string) = match rx_arc_mutex.try_lock() {
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

        let rx_arc_mutex = rx_arc_mutex.clone();

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
            println!(
                "Translate {} from {} to {} with {}",
                text, from_language, to_language, model
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
                    eprintln!("Unsupported language: {}", from_language);
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
                    eprintln!("Unsupported language: {}", from_language);
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
                    Box::new(backends::DeepSeekSentenceTranslator::new(api_key))
                }
                "youdao" => {
                    let (tx, rx) = mpsc::channel();
                    *rx_arc_mutex.lock().unwrap() = rx;
                    tx.send("Youdao api is not supported yet!!".into()).unwrap();
                    return;
                }
                "qwen" => {
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
                println!("Detect empty string, skip translating.");
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
