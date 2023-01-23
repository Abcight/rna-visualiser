#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod app;
use app::*;

mod protein_map;

fn main() {
	let native_options = eframe::NativeOptions::default();
	eframe::run_native(
		"Motorola Białka",
		native_options,
		Box::new(|cc| Box::new(App::new(cc))),
	);
}
