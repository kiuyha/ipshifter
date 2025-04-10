// "This tool was originally created by ByteBreach, who made Tornet.
// While the initial concept was admirable, I decided to take it to the next level by fixing a few... minor issues,
// such as causing the computer to restart every time you try to shut it down. I've refactored the code,
// improved the logic, and made sure the program runs efficiently.
// It's like taking the foundation of a house and turning it into a mansion, really."


use colored::*;
use clap::Parser;
use std::process::Command;
use std::{env, thread};
use std::time::Duration;
use std::process::Stdio;
use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;
use nix::unistd::Uid;

#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author = "Kiuyha")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A simple IP shifter tool that can change ip address in specific interval using TOR.")]

struct Args {
    #[arg(short='d', long="detach", help="Run in detached mode")]
    detached: bool,
    #[arg(short='s', long="stop", help="Run in detached mode")]
    stop: bool,
    #[arg(short='i', long="interval", default_value="10", help="Time in seconds between IP changes")]
    interval: u32,
    #[arg(short='c', long="count", default_value="0", help="Number of times to change the IP")]
    count: u32,
}

fn print_banner() {
    println!(
        " +----------------------------------------------------------------+",
    );
    println!(
        " |{}|",
        " ██╗██████╗ ███████╗██╗  ██╗██╗███████╗████████╗███████╗██████╗ ".green()
    );
    println!(
        " |{}|",
        " ██║██╔══██╗██╔════╝██║  ██║██║██╔════╝╚══██╔══╝██╔════╝██╔══██╗".green()
    );
    println!(
        " |{}|",
        " ██║██████╔╝███████╗███████║██║█████╗     ██║   █████╗  ██████╔╝".green()
    );
    println!(
        " |{}|",
        " ██║██╔═══╝ ╚════██║██╔══██║██║██╔══╝     ██║   ██╔══╝  ██╔══██╗".green()
    );
    println!(
        " |{}|",
        " ██║██║     ███████║██║  ██║██║██║        ██║   ███████╗██║  ██║".green()
    );
    println!(
        " |{}|",
        " ╚═╝╚═╝     ╚══════╝╚═╝  ╚═╝╚═╝╚═╝        ╚═╝   ╚══════╝╚═╝  ╚═╝".green()
    );
    println!(
        " +---------------------------{}{}{}-----------------------------+",
        "(".cyan(),
        "Kiuyha".red(),
        ")".cyan()
    );
}

fn sign_with_warning(warning: bool) -> String {
    if warning {
        format!("{}{}{}", "[".white(), "!".red(), "]".white())
    } else {
        format!("{}{}{}", "[".white(), "+".green(), "]".white())
    }
}

fn is_tor_installed() -> bool {
    let result = Command::new("tor")
        .arg("--version")
        .output()
        .expect("Failed to execute command");
    result.status.success()
} 

fn install_tor(){
    println!(" {} Installing Tor...",
        sign_with_warning(false),
    );
    let _result = Command::new("sudo")
        .args(["apt", "install", "-y", "tor"])
        .output(); 
    match _result {
        Ok(_output) => {
            println!(" {} Tor installed successfully.",
                sign_with_warning(false),
            );
        }
        Err(err) => {
            println!(" {} Error installing Tor: {}",
                sign_with_warning(true),
                err
            );
            program_stop(true);
        }
    }
}

fn start_tor(){
    let _result = Command::new("sudo")
        .args(["systemctl", "start", "tor"])
        .output(); 
    match _result {
        Ok(_output) => {
            println!(" {} Tor service started. Please wait a minute for Tor to connect.",
                sign_with_warning(false),
            );
            print_ip();
        }
        Err(err) => {
            println!(" {} Error starting Tor service: {}",
                sign_with_warning(true),
                err
            );
            println!(" {} Would you retry or shut down? (y/n): ",
                sign_with_warning(false)
            );
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            if input.trim() == "y" {
                start_tor();
            }else{
                program_stop(false);
            }
        }
    }
}

fn is_curl_installed() -> bool {
    let result = Command::new("curl")
        .arg("--version")
        .output()
        .expect("Failed to execute command");
    result.status.success()
}

fn install_curl(){
    println!(" {} Installing Curl...",
        sign_with_warning(false),
    );
    let _result = Command::new("sudo")
        .args(["apt", "install", "-y", "curl"])
        .output(); 
    match _result {
        Ok(_output) => {
            println!(" {} Curl installed successfully.",
                sign_with_warning(false),
            );
        }
        Err(err) => {
            println!(" {} Error installing Curl: {}",
                sign_with_warning(true),
                err
            );
            program_stop(true);
        }
    }
}

fn print_ip() {
    thread::sleep(Duration::from_secs(1));
    let result = Command::new("curl")
        .args(["--proxy", "socks5h://127.0.0.1:9050" ,"https://api.ipify.org"])
        .output();
    match result {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!(
                " {} Your IP has been changed to: {}",
                sign_with_warning(false),
                stdout.trim()
            );
        }
        Err(err) => {
            println!(" {} Error getting IP: {}",
                sign_with_warning(true),
                err
            );
            program_stop(true);
        }
    }
}

fn change_ip(interval: u32){
    thread::sleep(Duration::from_secs(interval.into()));
    let result = Command::new("sudo")
        .args(["systemctl", "restart", "tor"])
        .output();
    match result {
        Ok(output) => {
            if output.status.success() {
                print_ip();
            } else {
                eprintln!(
                    " {} Tor restart failed: {}",
                    sign_with_warning(true),
                    String::from_utf8_lossy(&output.stderr)
                );
                program_stop(true);
            }
        }
        Err(err) => {
            println!(" {} Error changing IP: {}",
                sign_with_warning(true),
                err
            );
            program_stop(true);
        }
    }
}

fn program_stop(err:bool){
    Command::new("sudo")
        .args(["systemctl", "stop", "tor"])
        .output()
        .expect("Failed to execute command");

    println!(" {} Program shutting down.",
        sign_with_warning(false),
    );

    change_system_proxy("127.0.0.1", 9050, false);

    if err {
        std::process::exit(1);
    }
    std::process::exit(0);
}

fn detect_desktop_environment() -> Option<String> {
    let output = Command::new("ps")
        .args(&["-e"])
        .output()
        .ok()?;
    
    let processes = String::from_utf8_lossy(&output.stdout);
    if processes.contains("gnome-shell") {
        Some("GNOME".to_string())
    } else if processes.contains("plasmashell") {
        Some("KDE".to_string())
    } else {
        None
    }
}

fn change_gnome_proxy(proxy_host: &str, proxy_port: u16, is_set: bool){
    Command::new("gsettings")
        .args(&[
            "set",
            "org.gnome.system.proxy",
            "mode",
            if is_set { "manual" } else { "none" },
        ])
        .status()
        .expect("Failed to set proxy mode");

    if is_set {
        // Set to manual proxy mode
        // Set socks5 proxy
        Command::new("gsettings")
            .args(&[
                "set",
                "org.gnome.system.proxy.socks",
                "host",
                proxy_host,
            ])
            .status()
            .expect("Failed to set proxy host");
    
        Command::new("gsettings")
            .args(&[
                "set",
                "org.gnome.system.proxy.socks",
                "port",
                &proxy_port.to_string(),
            ])
            .status()
            .expect("Failed to set proxy port");
    }
}

fn change_kde_proxy(proxy_host: &str, proxy_port: u16, is_set: bool) {
    let proxy_url = format!("socks5://{}:{}", proxy_host, proxy_port);
    Command::new("kwriteconfig5")
        .args([
            "--file", "kioslaverc",
            "--group","Proxy Settings",
            "--key", "ProxyType",
            if is_set { "1" } else { "0" },
        ])
        .status()
        .expect("Failed to change proxy");

    if is_set{
        Command::new("kwriteconfig5")
            .args([
                "--file", "kioslaverc",
                "--group","Proxy Settings",
                "--key", "socksProxy",
                &proxy_url,
            ])
            .status()
            .expect("Failed to set proxy");
    }
    Command::new("qdbus")
        .args([
            "org.kde.kded6",
            "/kded",
            "org.kde.kded6.reconfigure"
        ])
        .status()
        .expect("Failed to reconfigure kde");
}

fn change_system_proxy(proxy_host: &str, proxy_port: u16, is_set: bool) {
    let de = detect_desktop_environment().unwrap_or_default().to_lowercase();
    if de.contains("gnome") {
        change_gnome_proxy(proxy_host, proxy_port, is_set);
    } else if de.contains("kde") {
        change_kde_proxy(proxy_host, proxy_port, is_set);
    } else{
        println!(" {} Your desktop enviroment not supported, please set the proxy manually.",
            sign_with_warning(true),
        );
    }
    println!(" {} For using proxy with non gui applications, please set manually to socks5://{}:{}.",
        sign_with_warning(false),
        proxy_host,
        proxy_port
    );
}

fn check_connection(){
    if !is_curl_installed() {
        println!(" {} Curl is not installed.",
            sign_with_warning(true)
        );
        println!(" {} Would you like to install Curl? (y/n): ",
            sign_with_warning(false)
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() == "y" {
            install_curl();
        }
    }
    loop {
        thread::sleep(Duration::from_secs(1));
        let result = Command::new("curl")
            .arg("https://www.google.com")
            .output();

        match result {
            Ok(output) => {
                if !output.status.success() {
                    println!(
                        " {} Failed to connect to internet.",
                        sign_with_warning(true),
                    );
                    program_stop(true);
                }
            }
            Err(err) => {
                println!(
                    " {} Error connecting to internet: {}",
                    sign_with_warning(true),
                    err
                );
                program_stop(true);
            }
        }
    }
}

fn initialize(){
    println!(" {} Initializing...", 
        sign_with_warning(false),
    );
    if !is_tor_installed() {
        println!(" {} Tor is not installed.",
            sign_with_warning(true)
        );
        println!(" {} Would you like to install Tor? (y/n): ",
            sign_with_warning(false)
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() == "y" {
            install_tor();
        }
    }
    start_tor();
    change_system_proxy("127.0.0.1", 9050, true);
}

fn change_ip_repeatedly(interval: u32, count: u32){
    if count == 0 {
        loop {
            change_ip(interval);
        }
    }else{
        for _ in 0..count {
            change_ip(interval);
        }
        program_stop(false);
    }
}

fn main() {
    let args = Args::parse();

    // handling stop argument
    if args.stop{
        Command::new("pkill")
            .args(["-f","ipshifter"])
            .output()
            .expect("failed to stop ipshifter");
        std::process::exit(0);
    }

    // handling detached mode
    if args.detached {
        let exe = env::current_exe().expect("Failed to get current exe path");
        println!(" {} Running in detached mode.",
            sign_with_warning(false),
        );
        // run new process while make the output null
        Command::new(exe)
            .args(["-i", &args.interval.to_string(), "-c", &args.count.to_string()])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        std::process::exit(0);
    }

    // checking if user run it in root or sudo
    if Uid::effective().is_root() {
        eprintln!("Please not run this program as root");
        std::process::exit(1);
    }

    // handling termination signals
    let mut signals = Signals::new(&[SIGHUP, SIGINT, SIGTERM, SIGQUIT]).unwrap();

    // handling signals in a separate thread
    thread::spawn(move || {
        for _signal in signals.forever() {
            println!(" {} User terminated program.",
                sign_with_warning(true)
            );
            program_stop(false);
        }
    });

    print_banner();

    // checking connection in a separate thread
    thread::spawn(|| {
        check_connection();
    });

    initialize();
    change_ip_repeatedly(args.interval, args.count);
}
