wit_bindgen_guest_rust::generate!("plugin.wit");

struct Exports;

export_plugin!(Exports);

impl plugin::Plugin for Exports {
    fn hello_world() {
        let s = test::give_me_string();
        logging::log(&s);
    }

    fn hello_int(i: u32) {
        logging::log(&format!("HELLO INT {i}"));
    }

    fn hello_string(s: String) {
        logging::log(&format!("HELLO {s}"));
    }

    fn return_string(s: String) -> String {
        format!("RETURNING {s}")
    }
}
