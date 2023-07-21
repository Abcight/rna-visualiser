//! This is the main driver of the submission program.
//! It glues all of the structures declared in [`rnalib`]
//! together, while also providing a graphical user interface.
//!
//! Most of the types here are denoted with a lock icon, as
//! this crate does not export anything.

// #![windows_subsystem = "windows"]

/// A global memory allocator. This crate explicitly uses
/// mimalloc as its memory allocator, instead of the one
/// provided by the host operating system.
///
/// This guarantees better performance and consistency
/// across various platforms.
#[global_allocator]
#[cfg(not(target_arch = "wasm32"))]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

/// The byte array holding the program's icon to display
/// on title bar.
const ICON: &[u8] = include_bytes!("../icon/icon.bin");

use egui::*;

mod app;
use app::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
	let native_options = eframe::NativeOptions {
		icon_data: Some(eframe::IconData {
			rgba: ICON.to_vec(),
			width: 32,
			height: 32,
		}),
		initial_window_size: Some(Vec2::new(1280.0, 720.0)),
		vsync: true,
		..Default::default()
	};

	eframe::run_native(
		"Motorola Science Cup 2022/23 - Bioinformatyka",
		native_options,
		Box::new(|cc| Box::new(App::new(cc))),
	);
}

#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id",
                web_options,
                Box::new(|cc| Box::new(eframe_template::TemplateApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
