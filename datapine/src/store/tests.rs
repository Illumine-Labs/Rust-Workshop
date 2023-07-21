use super::Store;

#[test]
fn test_store(){
    let mut store = Store::new();
    store.insert("foo".to_string(), vec![1.0, 2.0, 3.0]);
    assert_eq!(store.get("foo"), Some(&vec![1.0, 2.0, 3.0]));
}

#[test]
fn test_store2(){
    let mut store = Store::new();
    store.insert("foo".to_string(), vec![1.0, 2.0, 3.0]);
    assert_eq!(store.get("foo"), Some(&vec![1.0, 2.0, 3.0]));
}

#[test]
fn test_remove(){
    let mut store = Store::new();
    store.insert("foo".to_string(), vec![1.0, 2.0, 3.0]);
    assert_eq!(store.remove("foo"), Some(vec![1.0, 2.0, 3.0]));
    assert_eq!(store.get("foo"), None);
}

#[test]
fn test_dump(){
    let mut store = super::Store::new();
    store.insert("foo".to_string(), vec![1.0, 2.0, 3.0]);
    assert_eq!(store.dump(), "{\"foo\":[1.0,2.0,3.0]}");
}