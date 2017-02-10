

pub struct ParserError2 {
    pub info: Option<ErrorInfo>,
    pub kind: ErrorKind
}

#[derive(Debug, PartialEq)]
pub enum ItemName {
    GenericError,
    MissingField,
    UnusedVariable,
}

pub struct Item {
    pub name: ItemName,
    pub kind: ItemKind,
    pub num: usize,
    pub title: &'static str,
}

pub enum ItemKind {
    Error,
    Warning,
}

pub struct Label {
    pub start_pos: usize,
    pub end_pos: usize,
    pub kind: LabelKind,
    pub text: &'static str,
}

pub enum LabelKind {
    Primary,
    Secondary,
}

pub static ITEMS: [Item; 2] = [Item {
                                   name: ItemName::GenericError,
                                   kind: ItemKind::Error,
                                   num: 1,
                                   title: "generic error",
                               },
                               Item {
                                   name: ItemName::MissingField,
                                   kind: ItemKind::Error,
                                   num: 2,
                                   title: "missing field {} in {}",
                               }];

pub fn get_item(name: ItemName) -> &'static Item {
    for x in &ITEMS {
        if x.name == name {
            return &x;
        }
    }
    panic!("Unknown item name: {:?}", name);
}
