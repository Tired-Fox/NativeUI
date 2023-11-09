use cypress_sys::windows::event::watch;
use cypress_sys::windows::window::Window;

fn main() {
    let window = Window::create("This is a sample window").unwrap();
    window.show();
    watch(window.handle());
}
