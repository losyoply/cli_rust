

use clap::{Command, Arg};
use clap::ColorChoice;
use CLI_rust::run_ls;

fn main() {
    let matches = Command::new("ls")
        .version("0.0.1")
        .author("losy")
        .about("ls in Rust")
        .color(ColorChoice::Always)
        .arg(
            Arg::new("path")
                .help("path to ls")
                .required(true)
                .index(1)
        )
        .get_matches();

    if let Some(path) = matches.get_one::<String>("path") {
        let output = run_ls(&path);

        for path in output {
            println!("Name: {}", path.unwrap().path().display())
        }
        
    } else {
        println!("No path provided");
    }
}
// fn main() {
//     let path = ".";
//     let paths = fs::read_dir(path).unwrap();
//     for path in paths {
//         println!("Name: {}", path.unwrap().path().display())
//     }
// }