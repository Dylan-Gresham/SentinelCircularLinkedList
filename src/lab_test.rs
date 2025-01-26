use crate::lab::List;
use std::rc::Rc;

// Moved to separate file under recommendation from Michael Olacsi

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
    assert_eq!(list.remove_index(0), Ok(0));
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
    assert_eq!(list.remove_index(3), Ok(1));
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
    assert_eq!(list.remove_index(4), Ok(0));
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
    for i in 0..5 {
        assert_eq!(list.remove_index(0), Ok(4 - i));
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
