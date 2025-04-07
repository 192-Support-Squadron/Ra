use serde::{Deserialize, Serialize};
use tera::{Context, Result, Tera};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Interface {
    pub name: String,
    pub range: Option<bool>,
    pub description: Option<String>,
    pub template: Option<String>,
    pub mtu: Option<String>,
    pub shutdown: Option<bool>,
    pub service_policies: Option<ServicePolicies>,
    pub switchport: Option<Switchport>,
    pub ip_address: Option<String>,
    pub ospf: Option<InterfaceOspfSettings>,
    pub vrrp: Option<VrrpSettingsV4>,
    pub dhcp_relays: Option<Vec<String>>,
    pub channel_group: Option<String>,
    pub vrf: Option<String>,
    pub tcp_mss: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePolicies {
    qos: Option<QosServicePolicies>,
    queuing: Option<QueuingServicePolicies>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct QosServicePolicies {
    input: Option<String>,
    output: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct QueuingServicePolicies {
    input: Option<String>,
    output: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Switchport {
    mode: SwitchportMode,
    trunk: Option<TrunkPortSettings>,
    access: Option<AccessPortSettings>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TrunkPortSettings {
    allowed_vlans: Option<String>,
    native_vlan: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPortSettings {
    vlan: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum SwitchportMode {
    Trunk,
    Access,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct System {
    pub domain: String,
    pub instance: String,
    pub site: String,
    pub router_id: Option<String>,
    pub site_ip: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Vlan {
    pub name: String,
    pub id: u32,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NetDevice {
    pub system: System,
    pub function: Option<String>,
    pub fabric: Option<String>,
    pub instance: Option<u32>,
    pub hostname: Option<String>,
    pub interfaces: Option<Vec<Interface>>,
    pub vlans: Option<Vec<Vlan>>,
    pub segment_routing: Option<SegmentRouting>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SubnetInfo {
    pub subnet: String,
    pub cidr: String,
    pub netmask: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SegmentRouting {
    pub global_sid: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VrrpSettingsV4 {
    pub primary_address: String,
    pub secondary_address: Option<String>,
    pub group: String,
    pub priority: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InterfaceOspfSettings {
    pub area: String,
    pub instance: String,
    pub passive: Option<bool>,
    pub network_type: Option<String>,
}

pub fn render_switch_templates(
    input_data: &String,
    fabric: &String,
    template: &String,
) -> Result<String> {
    let switch_input_data: Switch = serde_json::from_str::<Switch>(&input_data).unwrap();

    let template_directory: String = "./templates/".to_owned() + &fabric + "/**/*.tera";

    let tera: Tera = match Tera::new(&template_directory) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let context = &Context::from_serialize(&switch_input_data).unwrap();

    let template_name = template.to_owned() + ".tera";

    return tera.render(&template_name, &context);
}

pub fn render_router_templates(
    input_data: &String,
    fabric: &String,
    template: &String,
) -> Result<String> {
    let router_input_data: Router = serde_json::from_str::<Router>(&input_data).unwrap();

    let tera: Tera = match Tera::new("templates/**/*.tera") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let context = &Context::from_serialize(&router_input_data).unwrap();

    let template_name = fabric.to_owned() + "/" + &template + ".tera";

    return tera.render(&template_name, &context);
}
