use std::process::Command;
use Group13::manage_events;
use std::path::Path;
use auto_launch::AutoLaunchBuilder;
use std::env;

fn main() {
    let exe = env::current_exe().unwrap(); // exe path
    let len = exe.clone().to_str().unwrap().len();
    let wd: String = exe.to_str().unwrap().chars().take(len-7).collect::<Vec<char>>().into_iter().collect();
    // println!("{}", wd);

    /* Autostart configuration */
    let app_path = Path::new(&wd).join("Group13");
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
    let gui_path = Path::new(&wd).join("spawn_gui");
    // println!("{}", gui_path.to_str().unwrap());

    let mut gui = Command::new(gui_path)
        .spawn()
        .expect("Failed to execute process");


    /* Events startup */
    manage_events();

    gui.wait().expect("Failed to wait on child process");
    println!("Main executed successfully!")
}