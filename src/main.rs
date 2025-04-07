use serde_json;
use std::{fs, io::Write, path::Path};
pub mod deployment;
pub mod net_devices;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, short)]
    input_file: String,
}

fn main() {
    let cli = Cli::parse();

    let input_contents =
        fs::read_to_string(&cli.input_file)
            .expect("Should have been able to read the file");

    let input_template = serde_json::from_str::<deployment::Deployment>(&input_contents).unwrap();

    for device in input_template.devices {
        let rendered_device_input = deployment::render_device_input_templates(&device, &input_template.main).unwrap();

        let device_config = net_devices::render_net_device_templates(
                &rendered_device_input,
                &device.fabric,
                &device.template,
            );

        println!("{:?}", device_config);
        if let Ok(config) = device_config {
            let dir_path = "./Deployments/".to_owned() + &input_template.main.site + "/";

            // Check if the directory exists
            if !Path::new(&dir_path).exists() {
                // Create the directory if it doesn't exist
                match fs::create_dir_all(&dir_path) {
                    Ok(_) => println!("Directory created successfully"),
                    Err(e) => eprintln!("Failed to create directory: {}", e),
                }
            } else {
                println!("Directory already exists");
            }
            let mut rendered_device_config_file = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open(dir_path + &device.hostname + ".config")
                .unwrap();
            rendered_device_config_file
                .write_all(config.as_bytes())
                .unwrap();
            rendered_device_config_file.flush().unwrap();
        }
    }
}
