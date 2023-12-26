extern crate cypress_sys;

use cypress_sys::{
    event::{
        keyboard::{Key, KeyboardEvent},
        run, Event,
    },
    modal::Dialog,
    prelude::{WindowBuilder, WindowContext},
    Window,
};

fn main() {
    let _ = Window::builder().title("Rust Window").show().unwrap();

    // Run the Program, when the window opens `Escape` can be pressed to open a file select dialog
    run(|id, event, _| match event {
        Event::Keyboard(KeyboardEvent::KeyDown(key)) => match key {
            Key::Escape => {
                match Dialog::color()
                .initial(0xA0FF0C)
                // This will make the specified window the parent of this dialog
                .show_with(id)
                {
                    Ok(result) => println!("{:?}", result),
                    Err(error) => eprintln!("{}", error),
                }
            }
            _ => {}
        },
        _ => {}
    })
}
