use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

/// Custom type to make the `Node` struct more readable.
///
/// Shorthand for a Reference Counted pointer (`Rc`) holding a Reference Cell (`RefCell`) which keeps
/// track of how many References (`Ref<T>`) and Mutable References (`RefMut<T>`) exist of the stored
/// `Node<T>`.
pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;

/// A node in the list.
///
/// Each node will store data and links to the next and previous nodes.
#[derive(PartialEq, Debug)]
pub struct Node<T: PartialEq + Default + Display> {
    pub data: T,
    pub next: Link<T>,
    pub prev: Link<T>,
}

/// Struct to represent a list. The list maintains 1 function pointer to help with the management
/// of the data it is storing. This function must be provided by the user of this library.
#[derive(PartialEq, Debug)]
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
        if self.sentinel.borrow().next.is_some() {
            // If the sentinel's next node isn't the sentinel, update that node's prev pointer
            if !Rc::ptr_eq(
                &self.sentinel,
                &self.sentinel.borrow().next.clone().unwrap(),
            ) {
                self.sentinel
                    .borrow()
                    .next
                    .clone()
                    .unwrap()
                    .borrow_mut()
                    .prev = Some(Rc::clone(&new_node));
            }

            self.sentinel.borrow_mut().next = Some(Rc::clone(&new_node));
        }

        // Set the sentinel to point forwards to the new node
        self.sentinel.borrow_mut().next = Some(Rc::clone(&new_node));

        self.size += 1;
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
            Err(String::from("The list is empty, nothing was done"))
        } else if index >= self.size {
            Err(String::from("Index out of bounds"))
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

                    self.size -= 1;

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
    pub fn index_of(&self, data: T) -> Option<usize> {
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
                if node.borrow().data == data {
                    return Some(index);
                }

                // If we didn't find the target, and we're not on the sentinel node, increment
                current = node.borrow().next.clone();
                index += 1;
            }

            None
        }
    }
}

// This is Rust's version of toString
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

    #[test]
    fn test_add_one() {
        let mut list: List<usize> = List::new_list();

        // Add a new node
        list.add(0);

        // Assert that size was incremented
        assert_eq!(list.size, 1);

        // Assert that sentinel node still exists
        assert_eq!(list.sentinel.borrow().data, usize::default());

        // Check that the sentinel node now points to the new node in the next member
        assert!(!Rc::ptr_eq(
            &list.sentinel,
            &list.sentinel.borrow().next.clone().unwrap()
        ));

        // Check that the sentinel node now points to the new node in the prev member
        assert!(!Rc::ptr_eq(
            &list.sentinel,
            &list.sentinel.borrow().next.clone().unwrap()
        ));

        // Check that data is correct in the new node
        assert_eq!(list.sentinel.borrow().data, 0);

        // Check that the next and prev pointers are correct in the new node
        assert!(Rc::ptr_eq(
            &list
                .sentinel
                .borrow()
                .next
                .clone()
                .unwrap()
                .borrow()
                .next
                .clone()
                .unwrap(),
            &list.sentinel
        ));
        assert!(Rc::ptr_eq(
            &list
                .sentinel
                .borrow()
                .next
                .clone()
                .unwrap()
                .borrow()
                .prev
                .clone()
                .unwrap(),
            &list.sentinel
        ));
    }

    #[test]
    fn test_remove_index_zero() {
        let mut list: List<usize> = List::new_list();

        list.add(0);
        assert_eq!(list.remove_index(0), Ok(()));
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

    #[test]
    fn test_add_two() {
        let mut list: List<usize> = List::new_list();

        list.add(0);
        list.add(1);

        // List should be 1 -> 0 -> (sentinel)

        let sentinel = list.sentinel.borrow();
        let binding = sentinel.next.clone().unwrap();
        let one_node = binding.borrow();
        let binding_two = one_node.next.clone().unwrap();
        let zero_node = binding_two.borrow();

        // Assert that size was incremented properly
        assert_eq!(list.size, 2);

        // Assert that the nodes are in the proper order
        assert_eq!(one_node.data, 1);
        assert_eq!(zero_node.data, 0);
        assert_eq!(sentinel.next.clone().unwrap().borrow().data, one_node.data);
        assert_eq!(sentinel.prev.clone().unwrap().borrow().data, zero_node.data);
        assert_eq!(one_node.next.clone().unwrap().borrow().data, zero_node.data);
        assert_eq!(one_node.prev.clone().unwrap().borrow().data, sentinel.data);
        assert_eq!(zero_node.next.clone().unwrap().borrow().data, sentinel.data);
        assert_eq!(zero_node.prev.clone().unwrap().borrow().data, one_node.data);
    }

    #[test]
    fn test_remove_index_three() {
        let mut list: List<usize> = List::new_list();
        for i in 0..5 {
            list.add(i);
        }

        // List should be 4 -> 3 -> 2 -> 1 -> 0 -> (sentinel)

        assert_eq!(list.size, 5);
        assert_eq!(list.remove_index(3), Ok(()));
        assert_eq!(list.size, 4);

        // List should now be 4 -> 3 -> 2 -> 0 -> (sentinel)

        let sentinel = list.sentinel.borrow();
        let binding_one = sentinel.next.clone().unwrap();
        let four_node = binding_one.borrow();
        let binding_two = four_node.next.clone().unwrap();
        let three_node = binding_two.borrow();
        let binding_three = three_node.next.clone().unwrap();
        let two_node = binding_three.borrow();
        let binding_four = two_node.next.clone().unwrap();
        let zero_node = binding_four.borrow();

        // Assert that all the nodes have the correct values (ensuring that the correct node was
        // removed)
        assert_eq!(sentinel.data, usize::default());
        assert_eq!(four_node.data, 4);
        assert_eq!(three_node.data, 3);
        assert_eq!(two_node.data, 2);
        assert_eq!(zero_node.data, 0);

        // Assert that the order of nodes are correct
        assert_eq!(sentinel.next.clone().unwrap().borrow().data, four_node.data);
        assert_eq!(sentinel.prev.clone().unwrap().borrow().data, zero_node.data);
        assert_eq!(
            four_node.next.clone().unwrap().borrow().data,
            three_node.data
        );
        assert_eq!(four_node.prev.clone().unwrap().borrow().data, sentinel.data);
        assert_eq!(
            three_node.next.clone().unwrap().borrow().data,
            two_node.data
        );
        assert_eq!(
            three_node.prev.clone().unwrap().borrow().data,
            four_node.data
        );
        assert_eq!(two_node.next.clone().unwrap().borrow().data, zero_node.data);
        assert_eq!(
            two_node.prev.clone().unwrap().borrow().data,
            three_node.data
        );
        assert_eq!(zero_node.next.clone().unwrap().borrow().data, sentinel.data);
        assert_eq!(zero_node.prev.clone().unwrap().borrow().data, two_node.data);
    }

    #[test]
    fn test_remove_index_four() {
        let mut list: List<usize> = List::new_list();
        for i in 0..5 {
            list.add(i);
        }

        // List should be 4 -> 3 -> 2 -> 1 -> 0 -> (sentinel)

        assert_eq!(list.size, 5);
        assert_eq!(list.remove_index(4), Ok(()));
        assert_eq!(list.size, 4);

        // List should now be 4 -> 3 -> 2 -> 1 -> (sentinel)

        let sentinel = list.sentinel.borrow();
        let binding_one = sentinel.next.clone().unwrap();
        let four_node = binding_one.borrow();
        let binding_two = four_node.next.clone().unwrap();
        let three_node = binding_two.borrow();
        let binding_three = three_node.next.clone().unwrap();
        let two_node = binding_three.borrow();
        let binding_four = two_node.next.clone().unwrap();
        let one_node = binding_four.borrow();

        // Assert that all the nodes have the correct values (ensuring that the correct node was
        // removed)
        assert_eq!(sentinel.data, usize::default());
        assert_eq!(four_node.data, 4);
        assert_eq!(three_node.data, 3);
        assert_eq!(two_node.data, 2);
        assert_eq!(one_node.data, 1);

        assert_eq!(sentinel.next.clone().unwrap().borrow().data, four_node.data);
        assert_eq!(sentinel.prev.clone().unwrap().borrow().data, one_node.data);
        assert_eq!(
            four_node.next.clone().unwrap().borrow().data,
            three_node.data
        );
        assert_eq!(four_node.prev.clone().unwrap().borrow().data, sentinel.data);
        assert_eq!(
            three_node.next.clone().unwrap().borrow().data,
            two_node.data
        );
        assert_eq!(
            three_node.prev.clone().unwrap().borrow().data,
            four_node.data
        );
        assert_eq!(two_node.next.clone().unwrap().borrow().data, one_node.data);
        assert_eq!(
            two_node.prev.clone().unwrap().borrow().data,
            three_node.data
        );
        assert_eq!(one_node.next.clone().unwrap().borrow().data, sentinel.data);
        assert_eq!(one_node.prev.clone().unwrap().borrow().data, two_node.data);
    }

    #[test]
    fn test_invalid_index() {
        let mut list: List<usize> = List::new_list();
        for i in 0..5 {
            list.add(i);
        }

        // List should be 4 -> 3 -> 2 -> 1 -> 0 -> (sentinel)

        assert_eq!(list.size, 5);
        assert_eq!(
            list.remove_index(666),
            Err(String::from("Index out of bounds"))
        );
        assert_eq!(list.size, 5);

        // List should still be 4 -> 3 -> 2 -> 1 -> 0 -> (sentinel)

        let sentinel = list.sentinel.borrow();
        let binding_one = sentinel.next.clone().unwrap();
        let four_node = binding_one.borrow();
        let binding_two = four_node.next.clone().unwrap();
        let three_node = binding_two.borrow();
        let binding_three = three_node.next.clone().unwrap();
        let two_node = binding_three.borrow();
        let binding_four = two_node.next.clone().unwrap();
        let one_node = binding_four.borrow();
        let binding_five = one_node.next.clone().unwrap();
        let zero_node = binding_five.borrow();

        // Assert that all the nodes have the correct values (ensuring that no node was removed)
        assert_eq!(sentinel.data, usize::default());
        assert_eq!(four_node.data, 4);
        assert_eq!(three_node.data, 3);
        assert_eq!(two_node.data, 2);
        assert_eq!(one_node.data, 1);
        assert_eq!(zero_node.data, 0);

        // Assert that all the nodes are still in the correct order
        assert_eq!(sentinel.next.clone().unwrap().borrow().data, four_node.data);
        assert_eq!(sentinel.prev.clone().unwrap().borrow().data, zero_node.data);
        assert_eq!(
            four_node.next.clone().unwrap().borrow().data,
            three_node.data
        );
        assert_eq!(four_node.prev.clone().unwrap().borrow().data, sentinel.data);
        assert_eq!(
            three_node.next.clone().unwrap().borrow().data,
            two_node.data
        );
        assert_eq!(
            three_node.prev.clone().unwrap().borrow().data,
            four_node.data
        );
        assert_eq!(two_node.next.clone().unwrap().borrow().data, one_node.data);
        assert_eq!(
            two_node.prev.clone().unwrap().borrow().data,
            three_node.data
        );
        assert_eq!(one_node.next.clone().unwrap().borrow().data, zero_node.data);
        assert_eq!(one_node.prev.clone().unwrap().borrow().data, two_node.data);
        assert_eq!(zero_node.next.clone().unwrap().borrow().data, sentinel.data);
        assert_eq!(zero_node.prev.clone().unwrap().borrow().data, one_node.data);
    }

    #[test]
    fn test_remove_all() {
        let mut list: List<usize> = List::new_list();
        for i in 0..5 {
            list.add(i);
        }

        // Remove all nodes from the list
        for _ in 0..5 {
            assert_eq!(list.remove_index(0), Ok(()));
        }

        // Check that we're only left with the sentinel node
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

    #[test]
    fn test_index_of_zero() {
        let mut list: List<usize> = List::new_list();
        for i in 0..5 {
            list.add(i);
        }

        assert_eq!(list.size, 5);
        assert_eq!(list.index_of(0), Some(4));
    }

    #[test]
    fn test_index_of_three() {
        let mut list: List<usize> = List::new_list();
        for i in 0..5 {
            list.add(i);
        }

        assert_eq!(list.size, 5);
        assert_eq!(list.index_of(3), Some(1));
    }

    #[test]
    fn test_not_in_list() {
        let mut list: List<usize> = List::new_list();
        for i in 0..5 {
            list.add(i);
        }

        assert_eq!(list.size, 5);
        assert_eq!(list.index_of(22), None);
    }

    #[test]
    fn test_is_empty_true() {
        let list: List<usize> = List::new_list();

        assert_eq!(list.size, 0);
        assert_eq!(list.is_empty(), true);
    }

    #[test]
    fn test_is_empty_false() {
        let mut list: List<usize> = List::new_list();
        for i in 0..5 {
            list.add(i);
        }

        assert_eq!(list.size, 5);
        assert_eq!(list.is_empty(), false);
    }

    #[test]
    fn test_display_empty() {
        let list: List<usize> = List::new_list();

        assert_eq!(format!("{}", list), "(sentinel)\n");
    }

    #[test]
    fn test_display_with_elements() {
        let mut list: List<usize> = List::new_list();
        for i in 0..5 {
            list.add(i);
        }

        assert_eq!(format!("{}", list), "4 -> 3 -> 2 -> 1 -> 0 -> (sentinel)\n");
    }
}
