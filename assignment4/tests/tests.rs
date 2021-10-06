use assignment4::*;

#[test]
fn test_length() {
    let mut list = LinkedList::new();

    list.push(1);
    assert!(list.len() == 1);

    list.push(2);
    assert!(list.len() == 2);

    list.push_unique(3).unwrap();
    assert!(list.len() == 3);

    list.pop();
    assert!(list.len() == 2);
}

#[test]
fn test_values() {
    let mut list = LinkedList::new();

    list.push(String::from("string 1"));
    list.push(String::from("string 2"));

    assert_eq!(list.pop(), Some(String::from("string 2")));
    assert_eq!(list.pop(), Some(String::from("string 1")));
}

#[test]
fn test_none() {
    let mut list = LinkedList::new();

    list.push(1);
    list.pop();

    assert_eq!(list.pop(), None);
}



#[test]
fn test_errors() {
    let mut list = LinkedList::new();

    list.push_unique(2).unwrap();

    match list.push_unique(2) {
        Ok(()) => assert!(false),
        Err(_) => assert!(true)
    }
}

#[test]
fn test_print() {
    let mut list = LinkedList::new();
    list.push(100);

    println!("{}", list);

}
