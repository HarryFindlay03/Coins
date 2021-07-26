use rand::Rng;

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater,
    Empty,
}

fn main() {
    //create 5 coins of a random value and add all their cent values together. 
    let mut coins: [Coin; 5] = [Coin::Penny, Coin::Penny, Coin::Penny, Coin::Penny, Coin::Penny];

    for i in 0..coins.len() {
        let rnd_num = rand::thread_rng().gen_range(0..5);
        coins[i] = get_coin(rnd_num);
    }

    println!("{:?}", coins);
    println!("Total value of all coins: {}", total_value(coins));
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater => 25,
        _ => 0, //Coin::Empty
    }
}

fn total_value(coins: [Coin; 5]) -> u32 {
    let mut total = 0;
    for i in 0..coins.len() {
        total = total + value_in_cents(&coins[i])
        //&coins[i] so it doesn't move as Coin does not have a copy trait.
        //Passing as a reference so a move does not occur. 
    }
    total
}

fn get_coin(num :u8) -> Coin {
    if num == 1{
        Coin::Penny
    } else if num == 2 {
        Coin::Nickel
    } else if num == 3 {
        Coin::Dime
    } else if num == 4{
        Coin::Quater
    } else {
        println!("ERROR -> No more coins!");
        Coin::Empty
    }
}