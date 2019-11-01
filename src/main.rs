use std::ptr::null;

use libremarkable::appctx;

fn main() {
    env_logger::init();

    let mut app: appctx::ApplicationContext =
        appctx::ApplicationContext::new(on_button_press, on_wacom_input, on_touch_handler);

    app.clear(true);
}


fn on_button_press(app: &mut appctx::ApplicationContext, input: gpio::GPIOEvent) {}

fn on_wacom_input(app: &mut appctx::ApplicationContext, input: wacom::WacomEvent) {}

fn on_touch_handler(app: &mut appctx::ApplicationContext, input: multitouch::MultitouchEvent) {}
