fn main() {
	let mut current_number = 999*999;
	let mut num_string: String;
	let mut rev_string: String;

	'outer: loop {
		num_string = current_number.to_string();
		rev_string = num_string.chars().rev().collect();
		if num_string == rev_string {
			for i in (100..1000){
				if current_number % i == 0 {
					if ((current_number / i) >= 100) & ((current_number / i) <= 999){
						let num1 = i;
						let num2 = current_number / i;
						println!("Magic Number: {} First Number: {} Second Number: {}", current_number, num1, num2);
						break 'outer;
					}
				}
			}
		}
		current_number -= 1;
	}
}
