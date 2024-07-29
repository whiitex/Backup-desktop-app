#![windows_subsystem = "windows"]
use std::process::{Command, Stdio};
use Group13::manage_events;
use auto_launch::AutoLaunchBuilder;
use std::env;
use std::io::{Read, Stdout};

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


    /* Check if other Group13 processes exist*/
    let gui_path = wd.join("spawn_gui");

    #[cfg(target_os = "windows")]
    {
        let output = Command::new("tasklist")
            .args(&["/FI", "IMAGENAME eq Group13.exe","/NH"])
            .output()
            .expect("Failed to execute command");

        let already_active_proc = std::str::from_utf8(&output.stdout).unwrap().split("\n").count() -3;

        //println!("{:?}", already_active_proc);
        if already_active_proc > 0 {
            println!("Process already running!");
            return;
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let processes = Command::new("ps")
            .arg("-e")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute command 'ps'");
        let pid = Command::new("grep")
            .arg("Group13")
            .stdin(Stdio::from(processes.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute command 'grep'");

        let output = pid.wait_with_output().unwrap();
        let already_active_proc = std::str::from_utf8(&output.stdout).unwrap().split("\n").count() - 2;

        // println!("{:?}", already_active_proc);
        if already_active_proc > 0 {
            println!("Process already running!");
            return;
        }
    }


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