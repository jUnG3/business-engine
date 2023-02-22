use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use xml::name::OwnedName;
use xml::reader::XmlEvent;

use crate::model::Process;

pub type ProcessRef = Rc<RefCell<Process>>;

pub fn start_process(event: XmlEvent) -> Option<ProcessRef> {
    match event {
        XmlEvent::StartElement { name, attributes, .. } => {
            if name.local_name == "process" {
                let process = Rc::new(RefCell::new(Process::default()));
                for attribute in attributes {
                    let id = OwnedName::from_str("id").expect("Id should always be convertable");
                    if attribute.name == id {
                        process.borrow_mut().set_id(attribute.value);
                    }
                }
                Some(process)
            } else {
                None
            }
        }
        _ => { None }
    }
}
