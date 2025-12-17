use rand::random;

fn main() {
    for _ in 0..20{
        let x = rand::random_range(-50..50);
        let y = rand::random_range(-50..50);
        let z = rand::random_range(0.0..5.0);

        let r = rand::random_range(0..255);
        let g = rand::random_range(0..255);
        let b = rand::random_range(0..255);


    }



    println!("Hello, world!");
}
