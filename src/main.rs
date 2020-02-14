use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn checkout(items: Vec<&str>, price_map: HashMap<&str, u16>) -> u16 {
    let mut total = 0;
    let mut checkout_map: HashMap<&str, u16> = HashMap::new();
    let mut offers_map: HashMap<&str, u16> = HashMap::new();

    for item in items {
        let checkout_count: u16;
        checkout_count = match checkout_map.get(item) {
            Some(cost) => *cost,
            None => 1
        };

        checkout_map.insert(item, checkout_count);

        offers_map.entry(item).or_insert(0);

        total += match price_map.get(item) {
            Some(cost) => cost,
            None => &0
        };
        println!("Found {} in basket", item);
    }
    // for (item, price) in &price_map {
    //     println!("Price map \"{}\": {}", item, price);
    // }
    total
}

// fn scan_item(item_code: &str, price_map: HashMap<&str, u16>) -> &u16 {
//     match price_map.get(item_code) {
//         Some(price) => price,
//         None => &0
//     }
// }

#[test]
fn test_pos_spec_test_1 () {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!["B", "A", "B", "P", "B"], price_map);
    assert_eq!(checkout_test_result, 155);
}

#[test]
fn test_pos_spec_test_2() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!["A", "A"], price_map);
    assert_eq!(checkout_test_result, 50);
}

#[test]
fn test_pos_spec_test_3() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!["B", "A", "B", "P", "B", "A"], price_map);
    assert_eq!(checkout_test_result, 180);
}

#[test]
fn test_pos_spec_test_4() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!["B", "A", "B", "P", "B", "A", "P", "B", "B", "B"], price_map);
    assert_eq!(checkout_test_result, 310);
}

#[test]
fn test_pos_spec_test_5() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!["A", "A", "A", "A", "A"], price_map);
    assert_eq!(checkout_test_result, 100);
}

#[test]
fn test_pos_spec_test_6() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!["A", "A"], price_map);
    assert_eq!(checkout_test_result, 50);
}

#[test]
fn test_pos_spec_test_7() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!["A", "A", "A"], price_map);
    assert_eq!(checkout_test_result, 50);
}

#[test]
fn test_pos_spec_test_8() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!["A", "A", "A", "A", "A", "A", "A", "A"], price_map);
    assert_eq!(checkout_test_result, 150);
}

#[test]
fn test_pos_spec_test_9() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!["A", "A", "A", "A", "A", "A", "A", "A", "A"], price_map);
    assert_eq!(checkout_test_result, 150);
}

#[test]
fn test_pos_spec_test_10() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!["B", "B", "B"], price_map);
    assert_eq!(checkout_test_result, 100);
}

#[test]
fn test_pos_spec_test_11() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!["B", "B", "B", "B", "B", "B", "B", "B", "B", "B", "B", "B"], price_map);
    assert_eq!(checkout_test_result, 400);
}

#[test]
fn test_pos_spec_test_12() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!["B", "B", "B", "B", "B", "B", "B", "B", "B", "B", "B", "B", "B"], price_map);
    assert_eq!(checkout_test_result, 440);
}

#[test]
fn test_pos_spec_test_13() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!["B", "B", "B", "B", "B", "B", "B", "B", "B", "B", "B", "B", "B", "B"], price_map);
    assert_eq!(checkout_test_result, 480);
}

#[test]
fn test_pos_spec_test_14() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!["B", "B", "A", "P", "P", "B", "A", "A", "P", "A"], price_map);
    assert_eq!(checkout_test_result, 265);
}

#[test]
fn test_pos_spec_test_15() {
    let price_map: HashMap<&str, u16> = [("A", 25), ("B", 40), ("P", 30)].iter().cloned().collect();
    let checkout_test_result = checkout(vec!["B", "B", "A", "P", "P", "B", "A", "A", "P"], price_map);
    assert_eq!(checkout_test_result, 240);
}