use std::env;
use std::process::Command;
use auto_launch::{AutoLaunch, AutoLaunchBuilder};

fn main() {
    let exe = env::current_exe().unwrap(); // exe path
    let wd = exe.parent().unwrap();
    let app_path = wd.join("Group13");

    let _ = AutoLaunchBuilder::new()
        .set_app_name("Group13")
        .set_app_path(&app_path.to_str().unwrap())
        .set_use_launch_agent(false)
        .build()
        .unwrap().disable();

    #[cfg(target_os = "macos")]
    {
        let _ = AutoLaunchBuilder::new()
            .set_app_name("Group13")
            .set_app_path(&app_path.to_str().unwrap())
            .set_use_launch_agent(true)
            .build()
            .unwrap().disable();
    }

    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("taskkill")
            .args(&["/IM", "Group13.exe", "/F"])
            .output()
            .expect("Failed to execute command");
        let _ = Command::new("taskkill")
            .args(&["/IM", "spawn_gui.exe", "/F"])
            .output()
            .expect("Failed to execute command");
        let _ = Command::new("taskkill")
            .args(&["/IM", "spawn_popup.exe", "/F"])
            .output()
            .expect("Failed to execute command");
    }

    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("pkill")
            .args(&["-f", "Group13"])
            .output()
            .expect("Failed to execute command");
        let _ = Command::new("pkill")
            .args(&["-f", "spawn_gui"])
            .output()
            .expect("Failed to execute command");
        let _ = Command::new("pkill")
            .args(&["-f", "spawn_popup"])
            .output()
            .expect("Failed to execute command");
    }

    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("pkill")
            .args(&["-f", "Group13"])
            .output()
            .expect("Failed to execute command");
        let _ = Command::new("pkill")
            .args(&["-f", "spawn_gui"])
            .output()
            .expect("Failed to execute command");
        let _ = Command::new("pkill")
            .args(&["-f", "spawn_popup"])
            .output()
            .expect("Failed to execute command");
    }

}