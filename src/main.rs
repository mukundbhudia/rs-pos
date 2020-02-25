use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn checkout(items: Vec<&str>, price_map: HashMap<&str, u16>) -> u16 {
    let mut total: i16 = 0;
    let mut checkout_map: HashMap<&str, u16> = HashMap::new();
    let mut offers_map: HashMap<&str, u16> = HashMap::new();

    for item in items {
        let checkout_count = checkout_map.entry(item).or_insert(0);
        *checkout_count += 1;

        offers_map.entry(item).or_insert(0);

        let offers_map2: HashMap<&str, u16> = offers_map.clone();

        let apple_count = checkout_map.get("A").unwrap_or(&0);
        let banana_count = checkout_map.get("B").unwrap_or(&0);

        total += scan_item(item, &price_map) as i16;
        let checked_offers = check_offers(apple_count, banana_count, offers_map2, &price_map);
        total += checked_offers;
    }
    total as u16
}

#[allow(dead_code)]
fn check_offers(apple_count: &u16, banana_count: &u16, mut offers_map: HashMap<&str, u16>, price_map: &HashMap<&str, u16>) -> i16 {
    let mut total: i16 = 0;
    if apple_count % 3 == 0 && offers_map.contains_key("A") && apple_count / 3 > offers_map["A"] {
        total -= 25;
        let current_apple_offer = offers_map.entry("A").or_insert(0);
        // println!("current_apple_offer: {}", current_apple_offer);
        *current_apple_offer += 1;
    } else if banana_count % 3 == 0 && offers_map.contains_key("B") && banana_count / 3 > offers_map["B"] {
        total -= 20;
        let current_banana_offer = offers_map.entry("B").or_insert(0);
        // println!("current_banana_offer: {}", current_banana_offer);
        *current_banana_offer += 1;
    }
    // println!("Check offers total: {}", total);
    total
}

fn scan_item(item_code: &str, price_map: &HashMap<&str, u16>) -> u16 {
    *price_map.get(item_code).unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
