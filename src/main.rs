use xml::EventReader;
use xml::reader::XmlEvent;
use xml::reader::XmlEvent::StartElement;

mod model;
mod reader;

pub fn main() {
    let parser = EventReader::from_str(include_str!("test-process.bpmn"));
    for event in parser {
        match event {
            Ok(e) => {
                match e {
                    StartElement { name, attributes, namespace } => {
                        if name.local_name == "process" {

                        }
                    }
                    _ => {}
                }
            }
            Err(e) => { println!("Error happend {}", e); }
        }
    }
}
