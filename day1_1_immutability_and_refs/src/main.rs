fn main() {
    let x = 42;
    let r = &x;

    println!("r = {}", r); // r = 42

    let mut x = 42;
    let r = &mut x;

    func1(r);

    println!("r = {}", r); // r = 43
}

fn func1(a: &mut u32) {
    //*a = *a + 1;
    *a += 1;
    println!("a = {}", a); // a = 43
}


