use std::sync::Arc;

use backends::SentenceTranslator;

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

    main_window.global::<Logic>().on_translate_word(|text| {
        // Implement translation logic here
        println!("Translate Word: {}", text.to_uppercase());
        text.to_uppercase().into()
    });

    main_window.global::<Logic>().on_show_about_slint(move || {
        // Implement show about slint logic here
        println!("About Slint");
        about_slint_window.show().unwrap();
    });

    main_window
        .global::<Logic>()
        .on_show_setting_window(move || {
            println!("Show Setting Window");
            setting_window.show().unwrap();
        });

    // Translate sentence
    main_window.global::<Logic>().on_translate_sentence({
        let main_window_weak_arc = main_window_weak_arc.clone();

        move |text| {
            let text = text.to_string();
            println!("Translate {}", text);

            if text == String::new() {
                println!("Detect empty string, skip translating.");
                return;
            }

            let main_window_weak = main_window_weak_arc.clone();
            let main_window = main_window_weak.upgrade().unwrap();

            let api_key = main_window.get_api_key().to_string();

            // spawn a thread to avoid blocking the UI thread.
            std::thread::spawn(move || {
                println!("api-key: {}", api_key);
                let translator = backends::DeepSeekSentenceTranslator::new("sk-xx".to_string());

                let translate_result = translator
                    .translate_sentence(
                        &text,
                        backends::Language::English,
                        backends::Language::Chinese,
                    )
                    .unwrap_or_else(|e| format!("Translation failed: {}", e));

                // Update UI in the event loop
                let _ = main_window_weak.upgrade_in_event_loop(move |handle| {
                    handle.set_sentence_translate_result(translate_result.into())
                });
            });
        }
    });

    main_window.run()
}
