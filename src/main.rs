use colored::*;
use regex::Regex;
use std::io::{self, Write};
use webbrowser;

fn main() {

    //Banner
    print_banner();


    // Prompt for input
    print!("Enter an IP or hash: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    // Determine type
    let vt_url = if is_ip(input) {
        format!("https://www.virustotal.com/gui/ip-address/{}/detection", input)
    } else {
        // Assume hash
        format!("https://www.virustotal.com/gui/file/{}/detection", input)
    };

    println!("Opening: {}", vt_url);

    // Open in default browser
    if webbrowser::open(&vt_url).is_ok() {
        println!("URL opened in your browser.");
    } else {
        eprintln!("Failed to open browser.");
    }
}

fn is_ip(input: &str) -> bool {
    let ip_re =
        Regex::new(r"^(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.|$)){4}$").unwrap();
    ip_re.is_match(input)
}

fn print_banner() {
    let banner = r#"
____   _______________.____     ____ ___ 
\   \ /   /\__    ___/|    |   |    |   \
 \   Y   /   |    |   |    |   |    |   /
  \     /    |    |   |    |___|    |  / 
   \___/     |____|   |_______ \______/  
                              \/                                                                                                            
"#;
    println!("{}", banner.bright_blue().bold());
}