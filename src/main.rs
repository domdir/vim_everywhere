use inputbot::*;

fn main() -> Result<(), systray::Error> {
    // // Autorun for videogames.
    // NumLockKey.bind(|| {
    //     while NumLockKey.is_toggled() {
    //         LShiftKey.press();
    //         WKey.press();
    //         sleep(Duration::from_millis(50));
    //         WKey.release();
    //         LShiftKey.release();
    //     }
    // });

    // // Rapidfire for videogames.
    // RightButton.bind(|| {
    //     while RightButton.is_pressed() {
    //         LeftButton.press();
    //         sleep(Duration::from_millis(50));
    //         LeftButton.release();
    //     }
    // });

    // Send a key sequence.
    // KeybdKey::RKey.bind(|| KeySequence("Sample text").send());

    // // Move mouse.
    // KeybdKey::QKey.blockable_bind(|| {
    //     MouseCursor.move_rel(10, 10);
    //     BlockInput::Block
    // });

    // // Call this to start listening for bound inputs.
    // handle_input_events();
    set_tray()
}

fn set_tray() -> Result<(), systray::Error> {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }
    app.set_icon_from_file(&"..\\systray-rs\\resources\\rust.ico".to_string())?;
    app.set_tooltip(&"Whatever".to_string())?;
    // app.set_icon_from_file("/usr/share/gxkb/flags/ua.png")?;

    app.add_menu_item("Print a thing", |_| {
        println!("Printing a thing!");
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("Add Menu Item", |window| {
        window.add_menu_item("Interior item", |_| {
            println!("what");
            Ok::<_, systray::Error>(())
        })?;
        window.add_menu_separator()?;
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_separator()?;

    app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    println!("Waiting on message!");
    app.wait_for_message()?;
    Ok(())
}