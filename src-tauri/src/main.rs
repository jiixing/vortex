#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

fn main() {
  let context = tauri::generate_context!();
  let menu = tauri::Menu::new();

  let app_menu = Submenu::new(
    "Vortex",
    Menu::new()
      .add_item(CustomMenuItem::new("about", "About Vortex"))
      .add_native_item(MenuItem::Quit),
  );
  let file_menu = Submenu::new(
    "File",
    Menu::new()
      .add_item(CustomMenuItem::new("new", "New").accelerator("CmdOrControl+N"))
      .add_item(CustomMenuItem::new("open", "Open...").accelerator("CmdOrControl+O"))
      .add_native_item(MenuItem::Separator)
      .add_item(CustomMenuItem::new("save", "Save").accelerator("CmdOrControl+S"))
      .add_item(CustomMenuItem::new("saveas", "Save As..."))
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::CloseWindow)
      .add_native_item(MenuItem::Quit),
  );
  let edit_menu = Submenu::new(
    "Edit",
    Menu::new()
      .add_native_item(MenuItem::Undo)
      .add_native_item(MenuItem::Redo)
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Cut)
      .add_native_item(MenuItem::Copy)
      .add_native_item(MenuItem::Paste)
      .add_item(CustomMenuItem::new("selectall", "Select All").accelerator("CmdOrControl+A")),
  );
  let window_menu = Submenu::new(
    "Window",
    Menu::new()
      .add_native_item(MenuItem::Minimize)
      .add_native_item(MenuItem::Zoom),
  );

  tauri::Builder::default()
    .menu(
      menu
        .add_submenu(app_menu)
        .add_submenu(file_menu)
        .add_submenu(edit_menu)
        .add_submenu(window_menu),
    )
    // .menu(tauri::Menu::os_default(&context.package_info().name))
    .run(context)
    .expect("error while running tauri application");
}
