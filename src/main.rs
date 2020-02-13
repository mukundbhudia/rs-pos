use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn checkout(items: Vec<char>, price_map: HashMap<&str, u16>) -> u16 {
    0
}

#[test]
fn test_pos_spec_test () {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!['B', 'A', 'B', 'P', 'B'], price_map);
    assert_eq!(checkout_test_result, 155);
}

#[test]
fn test_pos_spec_test_2() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!['A', 'A'], price_map);
    assert_eq!(checkout_test_result, 50);
}

#[test]
fn test_pos_spec_test_3() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!['B', 'A', 'B', 'P', 'B', 'A'], price_map);
    assert_eq!(checkout_test_result, 180);
}

#[test]
fn test_pos_spec_test_4() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!['B', 'A', 'B', 'P', 'B', 'A', 'P', 'B', 'B', 'B'], price_map);
    assert_eq!(checkout_test_result, 310);
}

#[test]
fn test_pos_spec_test_5() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!['A', 'A', 'A', 'A', 'A'], price_map);
    assert_eq!(checkout_test_result, 100);
}

#[test]
fn test_pos_spec_test_6() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!['A', 'A'], price_map);
    assert_eq!(checkout_test_result, 50);
}

#[test]
fn test_pos_spec_test_7() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!['A', 'A', 'A'], price_map);
    assert_eq!(checkout_test_result, 50);
}

#[test]
fn test_pos_spec_test_8() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!['A', 'A', 'A', 'A', 'A', 'A', 'A', 'A'], price_map);
    assert_eq!(checkout_test_result, 150);
}

#[test]
fn test_pos_spec_test_9() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!['A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A'], price_map);
    assert_eq!(checkout_test_result, 150);
}

#[test]
fn test_pos_spec_test_10() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!['B', 'B', 'B'], price_map);
    assert_eq!(checkout_test_result, 100);
}

#[test]
fn test_pos_spec_test_11() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!['B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B'], price_map);
    assert_eq!(checkout_test_result, 400);
}

#[test]
fn test_pos_spec_test_12() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!['B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B'], price_map);
    assert_eq!(checkout_test_result, 440);
}

#[test]
fn test_pos_spec_test_13() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!['B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B'], price_map);
    assert_eq!(checkout_test_result, 480);
}

#[test]
fn test_pos_spec_test_14() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!['B', 'B', 'A', 'P', 'P', 'B', 'A', 'A', 'P', 'A'], price_map);
    assert_eq!(checkout_test_result, 265);
}

#[test]
fn test_pos_spec_test_15() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!['B', 'B', 'A', 'P', 'P', 'B', 'A', 'A', 'P'], price_map);
    assert_eq!(checkout_test_result, 240);
}