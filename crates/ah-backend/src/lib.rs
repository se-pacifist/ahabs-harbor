#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_window::init())
    .plugin(tauri_plugin_shell::init())
    .setup(|app| {
      //
      let mut window =
        tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("index.html".into()))
          .title("Ahabs Harbor")
          .inner_size(1100.0, 800.0)
          .center();
      #[cfg(desktop)]
      {
        window = window.title_bar_style(tauri::TitleBarStyle::Overlay);
        window = window.title("");
      }
      let main_window = window.build()?;
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
