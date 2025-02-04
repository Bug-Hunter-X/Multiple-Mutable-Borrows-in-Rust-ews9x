fn main() {
    let mut x = 5;
    { // Creating a new scope
        let y = &mut x;
        *y += 1;
    }
    { //Another new scope
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x); // x will be 7
}
