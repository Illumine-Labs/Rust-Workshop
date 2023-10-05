use std::process::Command;

use clap::{App, Arg};
use colored::*;
use std::thread::sleep;
use std::time::Duration;
use std::io::{stdout, Write};


fn paclear(color: &str, speed: i32) {
    let styled_pac = style(color, &PAC);
    let (rows, cols) = get_size();
    let width = PAC[0].len();
    let height = PAC.len();
    let speed = if speed < 1 { 1 } else { speed };
    let pitch = Duration::from_millis(20 / speed as u64);

    for y in (0..=rows-(height as i32)).step_by(height) {
        for x in (0..=cols-(width as i32)/3 ).step_by(1) {
            for (j , line) in styled_pac.iter().enumerate() {
                print!("\x1B[{};{}H{}", y + (j as i32) + 1, x, line);
            }
            stdout().flush().unwrap();
            sleep(pitch);
            for k in 0..height {
                print!("\x1B[{};{}H{}", y + (k as i32) + 1, x, " ".repeat(width));
            }
            stdout().flush().unwrap();
        }
    }

    // 清除屏幕
    print!("\x1B[2J");
    stdout().flush().unwrap();
}


fn style(color: &str, lines: &[&str]) -> Vec<String> {
    let mut styled = Vec::new();
    for line in lines.iter() {
        let styled_line = match color {
            "red" => line.red().to_string(),
            "green" => line.green().to_string(),
            "blue" => line.blue().to_string(),
            "yellow" => line.yellow().to_string(),
            "pink" => line.magenta().to_string(),
            _ => line.to_string(),
        };
        styled.push(styled_line);
    }
    styled
}


static PAC: [&str; 18] = [
    "	⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣤⣤⣤⣤⣤⣀⣀⠀⠀⠀⠀⠀⠀⠀",
	"	⠀⠀⠀⠀⠀⠀⠀⢀⣠⣶⡾⠿⠛⠋⠉⠉⠉⠉⠉⠙⠛⠿⢷⣦⣄⡀⠀⠀",
	"	⠀⠀⠀⠀⠀⣠⣶⠿⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠛⢿⣦⣀",
	"	⠀⠀⠀⢠⣾⠟⠁⠀⠀⠀⠀⠀⠀⣴⡿⠿⣶⡀⠀⠀⠀⠀⠀⠀⠀⣠⣿⠟",
	"	⠀⠀⣰⡿⠋⠀⠀⠀⠀⠀⠀⠀⠘⣿⣇⣀⣿⠇⠀⠀⠀⠀⢀⣠⣾⠟⠁⠀",
	"	⠀⣰⡿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠛⠛⠋⠀⠀⠀⢀⣴⡿⠛⠁⠀⠀⠀",
	"	⢠⣿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣶⡿⠋⠀⠀⠀⠀⠀⠀",
	"	⣸⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⠟⠉⠀⠀⠀⠀⠀⠀⠀⠀",
	"	⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣾⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
	"	⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⢿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
	"	⢹⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢿⣦⣀⠀⠀⠀⠀⠀⠀⠀⠀",
	"	⠘⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠿⣷⣄⠀⠀⠀⠀⠀⠀",
	"	⠀⠹⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⣷⣤⡀⠀⠀⠀",
	"	⠀⠀⠹⣿⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠛⢿⣦⡀⠀",
	"	⠀⠀⠀⠘⢿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⣿⣦",
	"	⠀⠀⠀⠀⠀⠙⠿⣶⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣾⠟⠉",
	"	⠀⠀⠀⠀⠀⠀⠀⠈⠙⠿⢷⣶⣤⣄⣀⣀⣀⣀⣀⣠⣤⣶⡾⠟⠋⠁⠀⠀",
	"	⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠛⠛⠛⠛⠛⠉⠉⠀⠀⠀⠀⠀⠀⠀",
];

fn get_size() -> (i32, i32) {
    let output = Command::new("stty").arg("size").output().expect("Failed to execute command");
    let s = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = s.split_whitespace().collect();
    
    if parts.len() < 2 {
        // 返回默认值或产生一个错误
        return (24, 80); // 默认的24x80大小
        // 或者可以使用以下代码产生一个错误：
        // panic!("Failed to get terminal size");
    }
    
    let rows = parts[0].parse::<i32>().unwrap_or(24);
    let cols = parts[1].parse::<i32>().unwrap_or(80);
    (rows, cols)
}



fn execute() {
    let matches = App::new("paclear")
        .version("0.1.0") // 示例版本
        .author("Your Name <your.email@example.com>")
        .about("paclear is a clear command with pacman animation")
        .arg(Arg::with_name("color")
            .short("c")
            .long("color")
            .value_name("COLOR")
            .help("Set pacman color (available: red, green, blue, yellow, pink)")
            .takes_value(true))
        .arg(Arg::with_name("speed")
            .short("s")
            .long("speed")
            .value_name("SPEED")
            .help("Set pacman multiple speed (default: 1)")
            .takes_value(true))
        .get_matches();

    // 获取颜色和速度参数
    let color = matches.value_of("color").unwrap_or("white");
    let speed: i32 = matches.value_of("speed").unwrap_or("1").parse().unwrap();

    // TODO: 根据参数执行相应的操作
    paclear(color,speed); // 示例调用
}

fn set_version_info(version: &str, date: &str) {
    // 在Rust中，我们可以直接在`App::new`中设置版本信息
    // 例如：.version(&format!("{} (Built on {})", version, date))
}

fn main() {
    execute();
}
