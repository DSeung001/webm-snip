mod commands;
mod ffmpeg;
mod models;

use commands::{
    choose_save_path, export_timeline, generate_default_output_path, open_file_path,
    open_folder_path, open_webm_files, probe_media,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_webm_files,
            probe_media,
            generate_default_output_path,
            choose_save_path,
            export_timeline,
            open_file_path,
            open_folder_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running WebM Snip");
}
