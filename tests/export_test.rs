use textfsm_rs::{OutputFormat, TextFSM, TextFsmExport};

#[test]
fn test_export_formats() {
    let template = r#"Value Name (\S+)
Value Age (\d+)

Start
  ^Name: ${Name}
  ^Age: ${Age} -> Record
"#;

    let data = "Name: Alice\nAge: 30\nName: Bob\nAge: 25\n";

    let mut fsm = TextFSM::from_string(template).unwrap();
    let results = fsm.parse_string(data, None).unwrap();

    // JSON
    let json = results.export(OutputFormat::Json).unwrap();
    assert!(json.contains("\"Name\": \"Alice\""));
    assert!(json.contains("\"Age\": \"30\""));

    // YAML
    let yaml = results.export(OutputFormat::Yaml).unwrap();
    assert!(yaml.contains("Name: Alice"));
    assert!(yaml.contains("Age: '30'"));

    // CSV
    let csv = results.export(OutputFormat::Csv).unwrap();
    // Headers are sorted alphabetically: Age,Name
    assert!(csv.contains("Age,Name"));
    assert!(csv.contains("30,Alice"));
    assert!(csv.contains("25,Bob"));

    // HTML
    let html = results.export(OutputFormat::Html).unwrap();
    assert!(html.contains("<table>"));
    assert!(html.contains("<th>Age</th>"));
    assert!(html.contains("<th>Name</th>"));
    assert!(html.contains("<td>Alice</td>"));

    // Text
    let text = results.export(OutputFormat::Text).unwrap();
    assert!(text.contains("Age  Name"));
    assert!(text.contains("---  ----"));
    assert!(text.contains("30   Alice"));

    // XML
    let xml = results.export(OutputFormat::Xml).unwrap();
    assert!(xml.contains("<results>"));
    assert!(xml.contains("<record>"));
    assert!(xml.contains("<Name>Alice</Name>"));
    assert!(xml.contains("<Age>30</Age>"));
    assert!(xml.contains("</record>"));
    assert!(xml.contains("</results>"));
}

#[test]
fn test_export_escaping() {
    let template = r#"Value MSG (.+)

Start
  ^Message: ${MSG} -> Record
"#;
    let data = "Message: <Hello> & \"World\"\n";

    let mut fsm = TextFSM::from_string(template).unwrap();
    let results = fsm.parse_string(data, None).unwrap();

    // XML
    let xml = results.export(OutputFormat::Xml).unwrap();
    assert!(xml.contains("&lt;Hello&gt; &amp; &quot;World&quot;"));

    // HTML
    let html = results.export(OutputFormat::Html).unwrap();
    assert!(html.contains("&lt;Hello&gt; &amp; &quot;World&quot;"));
}
