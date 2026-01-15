use std::io::Write;
use tempfile::NamedTempFile;
use textfsm_rs::TextFSM;

#[test]
fn test_basic_template() {
    let template = r#"Value Required INTERFACE (\S+)
Value STATUS (up|down)
Value IP (\d+\.\d+\.\d+\.\d+)

Start
  ^Interface ${INTERFACE} is ${STATUS}
  ^  IP address is ${IP} -> Record
"#;

    let data = r#"Interface GigabitEthernet0/1 is up
  IP address is 192.168.1.1
Interface GigabitEthernet0/2 is down
  IP address is 10.0.0.1
"#;

    let mut tmp_template = NamedTempFile::new().unwrap();
    write!(tmp_template, "{}", template).unwrap();

    let mut fsm = TextFSM::from_file(tmp_template.path()).unwrap();
    let result = fsm.parse_string(data, None).unwrap();

    assert_eq!(result.len(), 2);

    assert_eq!(
        result[0].fields.get("INTERFACE").unwrap().to_string(),
        "GigabitEthernet0/1"
    );
    assert_eq!(result[0].fields.get("STATUS").unwrap().to_string(), "up");
    assert_eq!(
        result[0].fields.get("IP").unwrap().to_string(),
        "192.168.1.1"
    );

    assert_eq!(
        result[1].fields.get("INTERFACE").unwrap().to_string(),
        "GigabitEthernet0/2"
    );
    assert_eq!(result[1].fields.get("STATUS").unwrap().to_string(), "down");
    assert_eq!(result[1].fields.get("IP").unwrap().to_string(), "10.0.0.1");
}

#[test]
fn test_continue_with_state_transition() {
    let _ = env_logger::builder().is_test(true).try_init();
    let template = r#"Value Required INTERFACE (\S+)
Value DESCRIPTION (.+)

Start
  ^interface ${INTERFACE} -> GetDescription

GetDescription
  ^ description ${DESCRIPTION} -> Record Start
  ^interface -> Continue.Record Start
  ^. -> Start
"#;

    let data = r#"interface GigabitEthernet0/1
 description Primary Uplink
interface GigabitEthernet0/2
 description Secondary Uplink
interface Loopback0
interface Vlan1
 description Management
"#;

    let mut tmp_template = NamedTempFile::new().unwrap();
    write!(tmp_template, "{}", template).unwrap();

    let mut fsm = TextFSM::from_file(tmp_template.path()).unwrap();
    let result = fsm.parse_string(data, None).unwrap();

    assert_eq!(result.len(), 4);
    assert_eq!(
        result[0].fields.get("INTERFACE").unwrap().to_string(),
        "GigabitEthernet0/1"
    );
    assert_eq!(
        result[0].fields.get("DESCRIPTION").unwrap().to_string(),
        "Primary Uplink"
    );

    assert_eq!(
        result[1].fields.get("INTERFACE").unwrap().to_string(),
        "GigabitEthernet0/2"
    );
    assert_eq!(
        result[1].fields.get("DESCRIPTION").unwrap().to_string(),
        "Secondary Uplink"
    );

    assert_eq!(
        result[2].fields.get("INTERFACE").unwrap().to_string(),
        "Loopback0"
    );
    assert_eq!(result[2].fields.get("DESCRIPTION").unwrap().to_string(), "");

    assert_eq!(
        result[3].fields.get("INTERFACE").unwrap().to_string(),
        "Vlan1"
    );
    assert_eq!(
        result[3].fields.get("DESCRIPTION").unwrap().to_string(),
        "Management"
    );
}
