use xml::reader::XmlEvent::StartElement;
use xml::EventReader;

mod model;
mod reader;

pub fn main() {
    let parser = EventReader::from_str(include_str!("test-process.bpmn"));
    for event in parser {
        match event {
            Ok(e) => match e {
                StartElement {
                    name,
                    attributes: _,
                    namespace: _,
                } => if name.local_name == "process" {},
                _ => {}
            },
            Err(e) => {
                println!("Error happend {}", e);
            }
        }
    }
}
