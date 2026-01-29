use std::io;

fn main() {

    let mut input1 = String::new();
    println!("-----------------------------------\n");
    println!("The SEC's Billing Calculator\n");
    println!("-----------------------------------");

    println!("Enter in the amount of energy consumed:");
    io::stdin().read_line(&mut input1).expect("Failed to readi input!");
    let consume_eneg: f32 = input1.trim().parse().expect("Please enter a valid number!");

    if consume_eneg > 0.0 && consume_eneg <= 100.0{
        println!("₦20 per kWh");
        let elect_bill:f32 = 20.0 *  consume_eneg;
        println!("Energy consumed: {:?} kWh",consume_eneg);
        println!("Total electricity bill: ₦{:?}",elect_bill);
    }
    else if consume_eneg > 100.0 && consume_eneg <= 200.0{
        println!("Above 100 kWh, now ₦25 per kWh");
        let elect_bill:f32 = 25.0 *  consume_eneg;
        println!("Energy consumed: {:?} kWh",consume_eneg);
        println!("Total electricity bill: ₦{:?}",elect_bill);
    }
    else if consume_eneg > 200.0{
        println!("Above 200 kWh, now ₦30 per kWh");
        let elect_bill:f32 = 30.0 *  consume_eneg;
        println!("Energy consumed: {:?} kWh",consume_eneg);
        println!("Total electricity bill: ₦{:?}",elect_bill);
    }
    else{
        println!("Error! Please try again!");
    }



}
