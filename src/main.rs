mod codestr;
mod config;
mod nginx_generator;
mod runall_generator;

use std::thread;
use std::{fs, time::Instant};

use argh::FromArgs;

use crate::{
    config::parse_config, nginx_generator::generate_nginx_file, runall_generator::generate_runall,
};

#[derive(FromArgs)]
/// The filepaths
struct CLIArguments {
    #[argh(positional)]
    /// configuration input filepath
    config_filepath: String,
    #[argh(positional)]
    /// nginx output config path
    nginx_filepath: String,
    #[argh(positional)]
    /// runall bash script output path
    runall_filepath: Option<String>,
}

fn main() {
    let start = Instant::now();

    let args: CLIArguments = argh::from_env();
    let configuration = parse_config(args.config_filepath);

    println!("Parsed configuration.");

    thread::scope(|scope| {
        scope.spawn(|| {
            let nginx = generate_nginx_file(&configuration);
            println!("Generated Nginx configuration.");
            fs::write(args.nginx_filepath, nginx).expect("Error while writing to Nginx file.");
            println!("Written Nginx configuration.");
        });

        if let Some(runall_path) = args.runall_filepath {
            scope.spawn(|| {
                let runall =
                    generate_runall(&configuration).expect("Error while generating runall file.");
                println!("Generated script.");
                fs::write(runall_path, runall).expect("Error while writing to runall file.");
                println!("Written script.");
            });
        } else {
            println!("Skipping script generation.");
        }
    });

    let duration = start.elapsed();
    println!(
        "Generated successfully in {} seconds.",
        duration.as_secs_f32()
    );
}
