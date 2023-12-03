use std::{rc::Rc, cell::RefCell};

use crate::maybe_part::MaybePart;

pub enum EngineSchematicSymbol {
    Empty,
    Symbol(char),
    Gear(u32),
    PartRef(Rc<RefCell<MaybePart>>),
}
