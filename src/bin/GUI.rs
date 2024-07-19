use Group13::BackupApp;

//use winapi::um::wincon::FreeConsole;
fn main() {
    println!("GUI is running");
    // unsafe {
    //     FreeConsole();
    // }


    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Back-up",
        native_options,
        Box::new(|cc| Box::new(BackupApp::new(cc))),
    ).expect("Terminated");

}