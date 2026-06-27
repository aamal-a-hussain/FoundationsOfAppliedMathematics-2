use std::fmt::{Debug};
use crate::DoublyLinkedListError;
use crate::DoublyLinkedListError::OutOfBounds;

#[derive(Debug)]
pub struct DoublyLinkedListNode<T> {
    value: T,
    prev: Option<usize>,
    next: Option<usize>
}

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    nodes: Vec<DoublyLinkedListNode<T>>,
    pub head: Option<usize>,
    pub tail: Option<usize>
}


impl <T: std::cmp::PartialEq> DoublyLinkedList<T> {
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
            prev: None,
            next: None
        };
        Self {
            nodes: vec![node],
            head: Some(0),
            tail: Some(0)
        }
    }

    fn find_recursive(&self, value: &T, idx: usize) -> Option<usize> {
        if idx >= self.nodes.len() {
            return None;
        }
        if self.nodes[idx].value == *value {
            return Some(idx);
        }

        if let Some(next) = self.nodes[idx].next {
            self.find_recursive(value, next)
        } else {
            None
        }
    }
    fn find_iterative(&self, value: &T) -> Option<usize> {
        let mut current = self.head;
        while let Some(idx) = current {
            if self.nodes[idx].value == *value { return Some(idx); }
            current = self.nodes[idx].next;
        };
        None
    }
    pub fn find(&self, value: &T) -> Option<usize> {
        if self.nodes.is_empty() {
            return None;
        }
        self.find_recursive(value, self.head.unwrap())
    }
    fn insert_prev(&mut self, index: usize, value: T) {
        let length = self.nodes.len();
        let new_next = &mut self.nodes[index];

        let node = DoublyLinkedListNode {
            value,
            prev: new_next.prev,
            next: Some(index)
        };
        new_next.prev = Some(length);
        self.nodes.push(node);
    }
    pub fn insert(&mut self, index: usize, value: T) -> Result<(), DoublyLinkedListError>{
        let length = self.nodes.len();
        if index > self.nodes.len() {
            return Err(OutOfBounds(
                format!("{} out of bounds for linked list of length {}", index, self.nodes.len())
            ))
        } else if index == length {
            self.push_back(value);
        } else if index == 0 {
            self.insert_prev(self.head.unwrap(), value);
            if self.tail.is_none() {
                self.tail = self.head;
            }
            self.head = Some(length);
        } else {
            let mut c_index = self.head.unwrap();
                for _ in 1..index+1 {
                    c_index = self.nodes[c_index].next.unwrap();
            }
            self.insert_prev(c_index, value);
        }

        Ok(())
    }

    pub fn push_back(&mut self, value: T) {
        let new_index = self.nodes.len();
        // Assign prev
        let prev: Option<usize> = match self.tail {
            Some(idx) => {
                // In this case there is a tail, so we point to that as prev and create a new tail
                self.tail = Some(new_index);
                self.nodes[idx].next = Some(new_index);
                Some(idx)
            },
            // If there is no tail assign the new value to both the head and tail
            None => {
                self.tail = Some(new_index);
                self.head = Some(new_index);
                None
            }
        };
        let node = DoublyLinkedListNode {
            value,
            prev,
            next: None,
        };

        self.nodes.push(node);
    }
}

#[cfg(test)]
mod tests {
    use crate::doubly_linked_list::{ DoublyLinkedList };

    fn create_template_list() -> DoublyLinkedList<f32> {
        let mut list = DoublyLinkedList::from_value(3.0f32);
        list.push_back(4.0);
        list
    }

    #[test]
    fn push_back_node_to_empty_list() {
        let mut list : DoublyLinkedList<f32>= DoublyLinkedList::new();
        list.push_back(3.0);
        assert_eq!(list.head, Some(0));
        assert_eq!(list.tail, list.head);
        assert_eq!(list.nodes[list.head.unwrap()].value, 3.0);
        assert_eq!(list.nodes[list.head.unwrap()].prev, None);
        assert_eq!(list.nodes[list.head.unwrap()].next, None);
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
        let mut list = create_template_list();
        list.push_back(5.0);

        assert_eq!(list.nodes.len(), 3);
        assert_eq!(list.head, Some(0));
        assert_eq!(list.tail, Some(2));

        assert_eq!(list.nodes[list.tail.unwrap()].value, 5.0);
        assert_eq!(list.nodes[list.tail.unwrap()].prev, Some(1));
        assert_eq!(list.nodes[list.tail.unwrap()].next, None);
    }

    #[test]
    fn insert_node_out_of_bounds() {
        let mut list = DoublyLinkedList::new();
        let result = list.insert(1, 3.0);
        assert!(result.is_err());
    }
    #[test]
    fn insert_node_at_head() {
        let mut list = DoublyLinkedList::from_value(3.0f32);
        let result = list.insert(0, 5.0);
        assert!(result.is_ok());

        assert_eq!(list.nodes.len(), 2);
        assert_eq!(list.head, Some(1));
        assert_eq!(list.tail, Some(0));

        assert_eq!(list.nodes[list.head.unwrap()].value, 5.0);
        assert_eq!(list.nodes[list.head.unwrap()].next, Some(0));
        assert_eq!(list.nodes[list.head.unwrap()].prev, None);
    }

    #[test]
    fn insert_node_in_list() {
        let mut list = create_template_list();
        let result = list.insert(1, 5.0);
        assert!(result.is_ok());

        assert_eq!(list.nodes.len(), 3);
        assert_eq!(list.head, Some(0));
        assert_eq!(list.tail, Some(1));

        assert_eq!(list.nodes[2].value, 5.0);
        assert_eq!(list.nodes[2].next, list.tail);
        assert_eq!(list.nodes[2].prev, list.head);

    }
    #[test]
    fn find_first_value_in_list() {
        let list = create_template_list();
        let value: f32 = 3.0;
        let result = list.find(&value);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn find_second_value_in_list() {
        let list = create_template_list();
        let value: f32 = 4.0;
        let result = list.find(&value);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 1);
    }
    #[test]
    fn find_value_in_empty_list() {
        let list = DoublyLinkedList::new();
        let value: f32 = 4.0;
        let result = list.find(&value);
        assert!(result.is_none());
    }

    #[test]
    fn find_non_existent_value() {
        let list = create_template_list();
        let value: f32 = 10.0;
        let result = list.find(&value);
        assert!(result.is_none());
    }
}
