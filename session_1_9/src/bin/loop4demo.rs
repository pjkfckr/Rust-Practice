

fn main() {
    let mut i = 3;
    loop {
        // Using special token print
        println!("{:?}", i);
        i = i - 1;
        if i == 0{
            break;
        }
    }
    println!("done");
}