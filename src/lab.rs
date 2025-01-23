use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

/// A node in the list.
#[derive(PartialEq)]
pub struct Node<T: PartialEq + Default + Display> {
    pub data: T,
    pub next: Link<T>,
    pub prev: Link<T>,
}

/// Struct to represent a list. The list maintains 1 function pointer to help with the management
/// of the data it is storing. This function must be provided by the user of this library.
#[derive(PartialEq)]
pub struct List<T: PartialEq + Default + Display> {
    pub size: usize,
    pub sentinel: Rc<RefCell<Node<T>>>,
}

impl<T: PartialEq + Default + Display> List<T> {
    /// Constructs a new list with a size of 0.
    ///
    /// ## Returns
    ///
    /// The properly typed equivalent of the following:
    ///
    /// ```rust
    /// List {
    ///     data: T::default(),
    ///     sentinel: Node {
    ///         data: T::default(),
    ///         prev: &sentinel,
    ///         next: &sentinel,
    ///     },
    /// }
    /// ```
    pub fn new_list() -> Self {
        let sentinel = Rc::new(RefCell::new(Node {
            data: T::default(),
            prev: None,
            next: None,
        }));

        sentinel.borrow_mut().next = Some(Rc::clone(&sentinel));
        sentinel.borrow_mut().prev = Some(Rc::clone(&sentinel));

        Self { size: 0, sentinel }
    }

    /// Determines if the list is empty.
    ///
    /// ## Returns
    ///
    /// 1. `true` if the list is empty.
    /// 2. `false` if the list isn't empty.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Adds data to the front of the list.
    ///
    /// ## Parameters
    ///
    /// - `data: &T` is a pointer to the data to add.
    pub fn add(&mut self, data: T) {
        // Put the input data into a Node
        let new_node = Rc::new(RefCell::new(Node {
            data,
            prev: Some(Rc::clone(&self.sentinel)),
            next: self.sentinel.borrow().next.clone(),
        }));

        // Set the current first non-sentinel node to point back to the new node instead of the
        // sentinel
        if let Some(first) = &self.sentinel.borrow().next {
            first.borrow_mut().prev = Some(Rc::clone(&new_node));
        }

        // Set the sentinel to point forwards to the new node
        self.sentinel.borrow_mut().next = Some(Rc::clone(&new_node));
    }

    /// Removes teh data at the specified index. If index is invalid then this function does
    /// nothing and returns `None`.
    ///
    /// ## Parameters
    ///
    /// - `index: usize` is the index of the data to remove.
    ///
    /// ## Returns
    ///
    /// 1. `Ok(())` if the target index was successfully removed.
    /// 2. `Err(String)` if the list is empty or the target index couldn't be found.
    pub fn remove_index(&mut self, index: usize) -> Result<(), String> {
        if self.is_empty() {
            return Err(String::from("The list is empty, nothing was done"));
        } else {
            let mut current = self.sentinel.borrow().next.clone();
            let mut i = 0;

            while let Some(node) = current {
                if Rc::ptr_eq(&node, &self.sentinel) {
                    break;
                }

                if i == index {
                    let prev = node.borrow().prev.clone();
                    let next = node.borrow().next.clone();

                    if let Some(prev_node) = prev.clone() {
                        prev_node.borrow_mut().next = next.clone();
                    }

                    if let Some(next_node) = next {
                        next_node.borrow_mut().prev = prev.clone();
                    }

                    return Ok(());
                }

                current = node.borrow().next.clone();
                i += 1;
            }

            Err(String::from(
                "The index couldn't be found, nothing was done.",
            ))
        }
    }

    /// Search for any occurrence of `data` from the list. Internally, this function will call
    /// `compare_to` on each item in the list until a match is found or the end of the list is
    /// reached. If there are multiple copies of the same data in the list, the first one will be
    /// returned.
    ///
    /// ## Parameters
    ///
    /// - `data: usize` is the data to search for.
    ///
    /// ## Returns
    ///
    /// 1. `Some(index)` where `index` is the location of the target data.
    /// 2. `None` if the target data couldn't be found.
    pub fn index_of(&self, data: &T) -> Option<usize> {
        if self.is_empty() {
            None
        } else {
            let mut index: usize = 0;
            let mut current = self.sentinel.borrow().next.clone();

            // While there exists a node
            while let Some(node) = current {
                // Break and return None if we wrap back around to the sentinel node without
                // finding our target data
                if Rc::ptr_eq(&node, &self.sentinel) {
                    break;
                }

                // If we found our target, return the current index
                if node.borrow().data == *data {
                    return Some(index);
                }

                // If we didn't find the target and we're not on the sentinel node, increment
                current = node.borrow().next.clone();
                index += 1;
            }

            None
        }
    }
}

impl<T: PartialEq + Default + Display> Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut print_str: String = String::new();
        let mut current = self.sentinel.borrow().next.clone();
        while let Some(node) = current {
            if Rc::ptr_eq(&node, &self.sentinel) {
                break;
            }

            print_str = format!("{}{} -> ", print_str, node.borrow().data);
            current = node.borrow().next.clone();
        }

        print_str = format!("{}(sentinel)\n", print_str);
        write!(f, "{}", print_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let list: List<usize> = List::new_list();

        assert_eq!(list.size, 0);
        assert_eq!(list.sentinel.borrow().data, usize::default());
        assert!(Rc::ptr_eq(
            &list.sentinel,
            &list.sentinel.borrow().next.clone().unwrap()
        ));
        assert!(Rc::ptr_eq(
            &list.sentinel,
            &list.sentinel.borrow().prev.clone().unwrap()
        ));
    }
}
