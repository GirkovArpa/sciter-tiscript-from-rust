extern crate sciter;
use sciter::{HELEMENT, Element, Value};
use std::env;
struct EventHandler {}
impl sciter::EventHandler for EventHandler {
	fn on_script_call(&mut self, root: HELEMENT, caller_function: &str, argv: &[Value]) -> Option<Value> {
		let _func = argv.first().unwrap().as_string().unwrap();
		let func = _func.as_str();
		&Element::from(root).call_function(func, argv);
		None
	}
}
fn main() {
	let mut frame = sciter::Window::new();
	let handler = EventHandler { };
	frame.event_handler(handler);
	let dir = env::current_dir().unwrap().as_path().display().to_string();
	let filename = format!("{}\\{}", dir, "index.htm");
	frame.load_file(&filename);
	frame.run_app();
}
