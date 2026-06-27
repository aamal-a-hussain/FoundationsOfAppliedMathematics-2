use std::fmt::Formatter;

mod doubly_linked_list;

#[derive(Debug)]
pub enum DoublyLinkedListError {
    OutOfBounds(String),
}

impl std::fmt::Display for DoublyLinkedListError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfBounds(err) => write!(f, "{err}")
        }
    }
}
