fn main() {
    let mut x = 0;
    let mut sum = 0;
    let max = 1000;

    while x < max {
    	if x % 3 == 0 {sum += x} else if x % 5 == 0 {sum += x};
    	x += 1;
    }
let sum_string = sum.to_string();

println!("{}",sum_string);

}
