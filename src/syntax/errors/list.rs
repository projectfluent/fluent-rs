#[derive(Debug, PartialEq)]
pub enum ItemName {
    GenericError,
    UnusedVariable
}

pub struct Item {
    pub name: ItemName,
    pub kind: ItemKind,
    pub num: usize,
    pub title: &'static str
}

pub enum ItemKind {
    Error,
    Warning,
}

pub static ITEMS: [Item; 2] = [
    Item {
        name: ItemName::GenericError,
        kind: ItemKind::Error,
        num: 1,
        title: "generic error"
    },
    Item {
        name: ItemName::UnusedVariable,
        kind: ItemKind::Warning,
        num: 1,
        title: "unused variable"
    }
];

pub fn get_item(name: ItemName) -> &'static Item {
    for x in &ITEMS {
        if x.name == name {
            return &x;
        }
    }
    panic!("Unknown item name: {:?}", name);
}
