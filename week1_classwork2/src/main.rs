use std::io;

fn discount_price(final_price:&f32) -> f32{
    if *final_price > 10000.0{
        let discounted = 0.15 * final_price;
        let form = final_price - discounted;
        return form;
    }
    else if *final_price > 5000.0{
        let discounted_2 = 0.1 * final_price;
        let form_2 = final_price - discounted_2;
        return form_2;
    }
    else{
        return *final_price;
    }
}

fn prices(code: &str) -> f32{
    match code.to_lowercase().as_str(){
        "c" => 1000.0,
        "m" => 1000.0,
        "l" => 2000.0,
        "a" => 2000.0,
        "e" => 5000.0,
        _ => 0.0
    }
}

fn coffees(code: &str) -> &str {
    match code.to_lowercase().as_str() {
        "c" => "Cappuccino",
        "m" => "Mocha",
        "l" => "Latte",
        "a" => "Americano",
        "e" => "Espresso",
        _ => "None"
    }
}
fn main() {


    println!("Welcome to Smart Cafe, where we 
    serve you the best coffee in town!");

    println!("\nThis is our menu:\n1.Cappucino(C) - 1000\n2.Mocha(M) - 1000\n3.Latte(L) - 2000\n4.Americano(A) - 2000\n5.Espresso(E) - 5000");

    println!("\nPlease what would you like to buy?\n");

    let mut total = 0.0;

    loop{
        let mut order = String::new();
        println!("(Please only enter the respective first letter of your choice \nand/or 'yes' when you are done ordering)");
        io::stdin().read_line(&mut order).expect("Unsuccessful!");
        let order_it =  order.trim();

        if order_it.eq_ignore_ascii_case("yes"){
            break;
        }

        let price = prices(order_it);
        if price == 0.0{
            println!("Invalid item. Try again.\n");
            continue;
        }

        let mut quantity = String::new();
        println!("How many {}(s) would you like?", coffees(order_it));
        io::stdin().read_line(&mut quantity).expect("Unsuccessful!");
        let quant:i32 = match quantity.trim().parse(){
            Ok(n) => n,
            Err(_) => {
                println!("Invalid quantity. Try again.\n");
                continue;
            }
        };

        let cost = price * quant as f32;
        total += cost;

        println!("Added {} x {} = ₦{:?}\n",quant,coffees(order_it),cost);

    }

    println!("\nOriginal Price: ₦{:?}",total);

    let x = discount_price(&total);

    if total > 10000.0{
        println!("\nDiscount Applied:15%");
    }
    else if total > 5000.0 {
        println!("\nDiscount Applied:10%");
    }
    else{
        println!("\nNo Discout Applied!");
    }

    println!("\nYour total bill is ₦{:?}",x);

    println!("\nThank you for visiting Smart Cafe!");

}
