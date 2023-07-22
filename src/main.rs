use std::process::Command;
use clap::{ Arg, App };

fn main() {
    let matches = App::new("app_remover")
        .version("1.0")
        .about("Removes an application")
        .arg(
            Arg::with_name("APP_NAME")
                .help("Sets the name of the application to remove")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::with_name("dry_run")
                .long("dry-run")
                .help("Performs a dry run (doesn't delete anything)")
                .takes_value(false)
        )
        .get_matches();

    let app_name = matches.value_of("APP_NAME").unwrap();
    let dry_run = matches.is_present("dry_run");

    let command = format!("/Applications/{}.app", app_name);

    let status = if dry_run {
        Command::new("echo").arg(format!("Would run: sudo rm -rf {}", command)).status()
    } else {
        Command::new("sudo").arg("rm").arg("-rf").arg(command).status()
    };

    match status {
        Ok(exit_status) => println!("Command executed with exit status: {}", exit_status),
        Err(err) => println!("Failed to execute command: {}", err),
    }

    let library_search_command = format!("find ~/Library -iname '*{}*'", app_name);

    let library_files = Command::new("sh")
        .arg("-c")
        .arg(library_search_command)
        .output()
        .expect("failed to execute process");

    println!("Potential related files/directories in ~/Library:");
    println!("{}", String::from_utf8_lossy(&library_files.stdout));
}
