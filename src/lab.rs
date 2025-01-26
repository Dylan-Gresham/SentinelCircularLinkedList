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
pub struct Node<T: PartialEq + Default + Display + Clone> {
    pub data: T,
    pub next: Link<T>,
    pub prev: Link<T>,
}

/// Struct to represent a list. The list maintains 1 function pointer to help with the management
/// of the data it is storing. This function must be provided by the user of this library.
#[derive(PartialEq, Debug)]
pub struct List<T: PartialEq + Default + Display + Clone> {
    pub size: usize,
    pub sentinel: Rc<RefCell<Node<T>>>,
}

impl<T: PartialEq + Default + Display + Clone> List<T> {
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

            // Removed unnecessary double sentinel.next update. Caught by Michael Olasci
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
    pub fn remove_index(&mut self, index: usize) -> Result<T, String> {
        if self.is_empty() {
            Err(String::from("The list is empty, nothing was done"))
        } else if index >= self.size {
            Err(String::from("Index out of bounds"))
        } else {
            let mut current = self.sentinel.borrow().next.clone();
            let mut i = 0;

            while let Some(node) = current {
                // Changed from a pointer equivalency check to an index/size check under suggestion
                // from Michael Olasci
                if i == self.size {
                    break;
                }

                if i == index {
                    let prev = node.borrow().prev.clone();
                    let next = node.borrow().next.clone();
                    let data = node.borrow().data.clone();

                    if let Some(prev_node) = prev.clone() {
                        prev_node.borrow_mut().next = next.clone();
                    }

                    if let Some(next_node) = next {
                        next_node.borrow_mut().prev = prev.clone();
                    }

                    self.size -= 1;

                    return Ok(data);
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
                //
                // Changed from a pointer equivalency check to an index/size check under suggestion
                // from Michael Olasci
                if index == self.size {
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
impl<T: PartialEq + Default + Display + Clone> Display for List<T> {
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
