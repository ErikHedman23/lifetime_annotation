// This is defining a generic lifetime - Lifetime annotation
// These must start with an (')
// Naming convention is to use a single lowercase letter.
// This is saying that the lifetime of the returned reference will be as long as the lifetime of the two input parameters, x and y.

fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}

fn main() {
    let result;
    let propellant1 = String::from("RP-1");
    let propellant2 = String::from("LNG");
    result = best_fuel(&propellant1, &propellant2);
    println!("result is {}", result);
}

// The three rules for implicit lifetimes
/*
1. Each input parameter that is a reference is assigned its own lifetime

2. If there is exactly one input lifetime, assign it to all output lifetimes

3. If there is a reference to self (&self) or a mutabable ref to self (&mut self) input parameter, its lifetime will be assigned to all output lifetimes.

*/
