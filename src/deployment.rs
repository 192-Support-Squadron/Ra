use serde::{Deserialize, Serialize};
use tera::{Context, Result, Tera};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Deployment {
    pub main: MainInfo,
    pub devices: Vec<DeviceInfo>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub input_name: String,
    pub template: String,
    pub fabric: String,
    pub hostname: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct IspOptions {
    pub pe_ip_address: String,
    pub pe_gw_address: String,
    pub isp_subnet: SubnetInfo,
    pub upload: String,
    pub download: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SubnetInfo {
    pub subnet: String,
    pub cidr: String,
    pub netmask: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MainInfo {
    pub site: String,
    pub site_ip: String,
    pub domain: String,
    pub isp_info: Option<IspOptions>,
}

pub fn render_device_input_templates(device: &DeviceInfo, main: &MainInfo) -> Result<String> {
    let template_directory = "./templates/inputs/".to_owned() + &device.template + "/*.tera";

    let tera: Tera = match Tera::new(&template_directory) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let context = &Context::from_serialize(&main).unwrap();

    let template_name = device.input_name.to_owned() + ".tera";

    return tera.render(&template_name.to_lowercase(), &context);
}
