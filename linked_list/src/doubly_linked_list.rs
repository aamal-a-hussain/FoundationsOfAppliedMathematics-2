use std::fmt;
use std::fmt::{Debug, Formatter};
use crate::DoublyLinkedListError;
use crate::DoublyLinkedListError::OutOfBounds;

#[derive(Debug)]
pub struct DoublyLinkedListNode<T> {
    value: T,
    index: usize,
    prev: Option<usize>,
    next: Option<usize>
}

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    nodes: Vec<DoublyLinkedListNode<T>>,
    pub head: Option<usize>,
    pub tail: Option<usize>
}


impl <T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            head: None,
            tail: None
        }
    }
    pub fn from_value(value: T) -> Self {
        let node = DoublyLinkedListNode {
            value,
            index: 0,
            prev: None,
            next: None
        };
        Self {
            nodes: vec![node],
            head: Some(0),
            tail: None
        }
    }

    pub fn insert(&mut self, index: usize, value: T) -> Result<(), DoublyLinkedListError>{
        if index > self.nodes.len() {
            return Err(OutOfBounds(
                format!("{} out of bounds for linked list of length {}", index, self.nodes.len())
            ))
        } else if index == self.nodes.len() {
            self.push_back(value);
        } else if index == 0 {
            let node = DoublyLinkedListNode {
                value,
                index: 0,
                prev: None,
                next: self.head
            };
            let head = &mut self.nodes[self.head.unwrap()];
            head.prev = Some(0);
            self.nodes.push(node);
        }

        Ok(())
    }

    pub fn push_back(&mut self, value: T) {
        let new_index = self.nodes.len();
        // Assign prev
        let prev: Option<usize> = match self.tail {
            Some(idx) => {
                // In this case there is a tail, so we point to that as prev and create a new tail
                self.tail = Option::from(new_index);
                Option::from(idx)
            },
            None => {
                match self.head {
                    Some(_) => {
                        // In this case, there is a head, but no tail so create a tail
                        self.tail = Option::from(new_index);
                        self.head
                    },
                    None => {
                        // In this case there is no head so create the head first
                        self.head = Option::from(new_index);
                        None
                    }
                }
            }
        };
        let node = DoublyLinkedListNode {
            value,
            index: new_index,
            prev,
            next: None,
        };

        self.nodes.push(node);
    }
}

#[cfg(test)]
mod tests {
    use crate::doubly_linked_list::{ DoublyLinkedList, DoublyLinkedListNode };
    use crate::DoublyLinkedListError::OutOfBounds;

    #[test]
    fn push_back_node_to_empty_list() {
        let mut list : DoublyLinkedList<f32>= DoublyLinkedList::new();
        list.push_back(3.0);
        assert_eq!(list.head, Some(0));
        assert_eq!(list.nodes[list.head.unwrap()].value, 3.0);
        assert_eq!(list.nodes[list.head.unwrap()].prev, None);
        assert_eq!(list.nodes[list.head.unwrap()].next, None);

        assert_eq!(list.tail, None);
    }

    #[test]
    fn push_back_node_to_single_item_list() {
        let mut list = DoublyLinkedList::from_value(3.0f32);
        list.push_back(4.0);
        assert_eq!(list.nodes.len(), 2);

        assert_eq!(list.head, Some(0));
        assert_eq!(list.tail, Some(1));

        assert_eq!(list.nodes[list.tail.unwrap()].value, 4.0);
        assert_eq!(list.nodes[list.tail.unwrap()].prev, list.head);
        assert_eq!(list.nodes[list.tail.unwrap()].next, None);
    }

    #[test]
    fn push_back_node_to_multi_item_list() {
        let mut list = DoublyLinkedList::from_value(3.0f32);
        list.push_back(4.0);
        list.push_back(5.0);


        assert_eq!(list.nodes.len(), 3);
        assert_eq!(list.head, Some(0));
        assert_eq!(list.tail, Some(2));

        assert_eq!(list.nodes[list.tail.unwrap()].value, 5.0);
        assert_eq!(list.nodes[list.tail.unwrap()].prev, Some(1));
        assert_eq!(list.nodes[list.tail.unwrap()].next, None);

    }

    #[test]
    fn insert_node_out_of_bounds {
        let mut list = DoublyLinkedList::new();
        assert_eq!(list.insert(1, 3.0), Err(OutOfBounds("1 out of bounds for linked list of length 0")));
    }
}
