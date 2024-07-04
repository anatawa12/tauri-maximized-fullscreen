#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, RunEvent};

fn main() {
    tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| {
            if let RunEvent::WindowEvent { event, label, .. } = event {
                if let tauri::WindowEvent::Resized(size) = event {
                    let window = app.get_window(&label).expect("failed to get window");
                    let maximized = window.is_maximized().expect("failed to get maximized");
                    let fullscreen = window.is_fullscreen().expect("failed to get fullscreen");

                    window
                        .emit(
                            "log",
                            format!(
                                "Resized to {}x{}\nfullscreen: {}, maximized: {}",
                                size.width, size.height, fullscreen, maximized
                            ),
                        )
                        .expect("failed to resize");
                }
            }
        });
}
