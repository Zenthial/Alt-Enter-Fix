use std::fs;
use std::path::Path;
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;
use read_input::prelude::*;

fn delete_old_folder(path: String) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_name() == "ClientSettings" {
            fs::remove_dir(entry.path())?;
            break;
        }
    }
    Ok(())
}

fn settings_update(version_folder: String) -> std::io::Result<()> {
    fs::create_dir(version_folder.clone()+"/ClientSettings")?;
    fs::write(version_folder+"/ClientSettings/ClientAppSettings.json", "{\"FFlagHandleAltEnterFullscreenManually\":\"False\"}")?;
    Ok(())
}

fn find_version_folder(path: String) -> std::io::Result<String> {
    let entries = fs::read_dir(path)?;
    let mut longest_length: Option<SystemTime> = None;
    let mut longest_path: Option<PathBuf> = None;
    for entry in entries {
        let entry = entry?;
        if entry.file_name().into_string().unwrap().find("version").is_none() {
            continue;
        };
        let metadata = entry.metadata().unwrap();
        let length = metadata.modified().unwrap();
        if longest_length.is_none() && longest_path.is_none() {
            longest_length = Some(length);
            longest_path = Some(entry.path());
        } else {
            if longest_length.unwrap().le(&length) {
                longest_length = Some(length);
                longest_path = Some(entry.path());
            }
        }
    }
    let path = longest_path.unwrap();
    let path_str = path.to_str().unwrap();
    Ok(path_str.to_string())
}

fn check_path(path: String) -> Result<bool, io::Error> {
    let mut result = false;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_name() == "RobloxStudioLauncherBeta.exe" {
            result = true;
            break;
        }
    }
    Ok(result)
}

fn run(path: String) {
    let version_folder = find_version_folder(path.clone()).expect("Error finding version folder. Please send this to tomspell");
    let delete_old = delete_old_folder(path);
    if let Err(e) = delete_old {
        println!("Error deleted old folder: {}\nPlease send to tomspell", e);
        return
    }
    let res = settings_update(version_folder);
    if let Err(e) = res {
        println!("Error writing ClientSettings: {}\nPlease send to tomspell", e);
        return
    }
    println!("ClientSettings updated!")

}

fn main() {
    let path_file = Path::new("path.txt");
    if !path_file.exists() {
        let mut path = input::<String>().msg("This seems to be your first time using alt_enter! Please input your ROBLOX versions path.
        \n\nThis should be located in your local app data for most users.
        \nYou can find this by opening your roblox's file location until you reach the version folder.
        \nAt this point, you should navigate back one to the Versions directory.
        \nThis path should end with Roblox/Versions. 
        \nRight click the address bar and click \"Copy Address as Text\". Paste that here.
        \n\nDon't worry, you should only have to do this once.\n").get();

        path = path.replace("\\", "/");
        if check_path(path.clone()).unwrap() {
            println!("Path saved!");
            fs::write("path.txt", path.clone()).unwrap();
            run(path);
        } else {
            println!("Incorrect path, please try again!");
        }
    } else {
        let path: String = String::from_utf8_lossy(&fs::read("path.txt").unwrap()).parse().unwrap();
        run(path);
    }
    input::<String>().msg("Press any key to close.").get();
}
