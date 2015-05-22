fn main() {
    let mut num1 = 1;
    let mut num2 = 2;
    let mut newnum = 0;
    let mut total = 2; // start at 2 in order to cover initial set
    let max = 4000000;

    while newnum < max {
    	newnum = num1 + num2;
    	num1 = num2;
    	num2 = newnum;
    	if newnum % 2 == 0 {total += newnum};
    }

    let num_string = total.to_string();
    println!("{}", num_string);
}
