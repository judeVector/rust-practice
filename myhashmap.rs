use std::collections::HashMap;

pub fn test_hashmap_basic() {
    let mut stock_list: HashMap<String, f32> = HashMap::new();

    stock_list.insert("NVDA".to_string(), 478.32);
    stock_list.insert("GOGL".to_string(), 500.17);
    stock_list.insert("TSLA".to_string(), 150.87);
    // println!("{:#?}", stock_list);

    stock_list.remove(&("GOGL".to_string()));
    stock_list.insert("TSLA".to_string(), 250.89);
    stock_list.entry("AAPL".to_string()).or_insert(600.23);

    println!("{:#?}", stock_list);
    println!("{}", stock_list.len());
    println!("{}", stock_list.is_empty());

    for (ticker, price) in &stock_list {
        println!("Ticker: {} is trading at price: {}", ticker, price)
    }
}
