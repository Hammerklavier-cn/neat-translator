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
    let main_window = MainWindow::new()?;

    let about_slint_window = AboutSlintWindow::new()?;

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

    main_window.global::<Logic>().on_translate_sentence(|text| {
        // Implement translation logic here
        println!("Translate Sentence: {}", text.to_uppercase());
        text.to_uppercase().into()
    });

    main_window.run()
}
