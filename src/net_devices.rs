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
pub struct Switch {
    pub system: System,
    pub switch_type: Option<String>,
    pub instance: Option<u32>,
    pub hostname: Option<String>,
    pub interfaces: Option<Vec<Interface>>,
    pub vlans: Vec<Vlan>,
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
pub struct Router {
    pub system: System,
    pub router_type: Option<String>,
    pub instance: Option<u32>,
    pub hostname: Option<String>,
    pub interfaces: Option<Vec<Interface>>,
    pub segment_routing: Option<SegmentRouting>,
    pub isp_circuit_info: Option<IspOptions>,
    pub vlans: Option<Vec<Vlan>>,
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
