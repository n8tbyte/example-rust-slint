#![windows_subsystem = "windows"]

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    unsafe {
        std::env::set_var("SLINT_BACKEND", "software");
    }

    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();

    ui.on_say_hello(move |name| {
        let ui = ui_handle.unwrap();
        if name.is_empty() {
            ui.set_user_name("กรุณาใส่ชื่อ".into());
        } else {
            let greeting = format!("ยินดีที่ได้รู้จัก, {}", name);
            ui.set_user_name(greeting.into());
        }
        println!("User clicked the button with name: {}", name);
    });

    ui.run()
}

