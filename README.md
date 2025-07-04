# `main.rs`

```rs
    let _ = Product::init_storage();
    let _ = Cart::init_storage();

    let _ = Product::create(Product{
        name: "Cheese".to_string(),
        price: 12u64,
        quntity: 10,
        expiry_date: Some(ExpirationDate::new(Local::now() + Duration::days(1))),
        shipping_info: Some(ShippingWeight::new(450.0)),
    });

    let _ = Product::create(Product{
        name: "Mobile".to_string(),
        price: 12u64,
        quntity: 10,
        expiry_date: None,
        shipping_info: Some(ShippingWeight::new(650.0)),
    });

    let cheese = Product::get(&"Cheese".to_string()).unwrap();
    let mobile = Product::get(&"Mobile".to_string()).unwrap();

    let cheese_item = CartItem::from_product(4, cheese.clone());
    let mobile_item = CartItem::from_product(4, mobile.clone());

    let _ = Cart::add_item(cheese_item.clone());
    let _ = Cart::add_item(cheese_item.clone());

    Cart::checkout();
```


# Command Line

```sh
cargo run
```

# Output

```txt
** Shipment notice **
8x Cheese          3600.00g
Total package weight 3600g

** Checkout receipt **
8x Cheese          96   

--------------------
Subtotal       96   
Shipping       50.00
Total          146.00
```