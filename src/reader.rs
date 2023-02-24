use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use xml::name::OwnedName;
use xml::reader::XmlEvent;

use crate::model::Process;

pub type ProcessRef = Rc<RefCell<Process>>;

pub fn start_process(event: XmlEvent) -> Option<ProcessRef> {
    match event {
        XmlEvent::StartElement {
            name, attributes, ..
        } => {
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
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use xml::{attribute::OwnedAttribute, namespace::Namespace};

    use super::*;

    #[test]
    fn test_start_process_returns_process() {
        let mut attr: Vec<OwnedAttribute> = Vec::new();
        attr.push(OwnedAttribute::new(
            OwnedName::from_str("id").expect("This should never fail"),
            "42",
        ));
        let input = XmlEvent::StartElement {
            name: OwnedName::from_str("process").expect("This should never fail"),
            attributes: attr,
            namespace: Namespace::empty(),
        };
        let result = start_process(input);
        assert!(result.is_some());
        let result_process = result.expect("This should never fail");
        let a = result_process.borrow();
        assert_eq!("42", a.get_id())
    }
}
