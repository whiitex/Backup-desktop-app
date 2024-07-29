use std::process::Command;
use Group13::manage_events;
use std::path::Path;
use auto_launch::AutoLaunchBuilder;
use std::env;

fn main() {
    let exe = env::current_exe().unwrap(); // exe path
    let wd = exe.parent().unwrap();
    // println!("{}", wd);

    /* Autostart configuration */
    let app_path = wd.join("Group13");
    // println!("{}", app_path.to_str().unwrap());

    let auto = AutoLaunchBuilder::new()
        .set_app_name("Group13")
        .set_app_path(&app_path.to_str().unwrap())
        .set_use_launch_agent(true)
        .build()
        .unwrap();

    auto.enable().unwrap();
    println!("Autostart enabled: {}", auto.is_enabled().unwrap());


    /* App startup */
    let gui_path = wd.join("spawn_gui");
    // println!("{}", gui_path.to_str().unwrap());

    let mut gui = Command::new(gui_path)
        .spawn()
        .expect("Failed to execute process");


    /* Events startup */
    manage_events();

    gui.wait().expect("Failed to wait on child process");
    println!("Main executed successfully!")
}