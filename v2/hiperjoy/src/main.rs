use gtk::prelude::*;
use gtk::{glib, Application};
use hiperjoy::leap_hand_sensor::leap_hand_sensor;
use hiperjoy::build_ui::build_ui;

const APP_ID: &str = "HIPERJOY";


fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}


// fn main() {
//     leap_hand_sensor();
// }
