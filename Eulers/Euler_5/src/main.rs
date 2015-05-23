fn main() {
	let mut current_num = 20;
	'outer: loop{
		'inner: for i in 1..21{
			if current_num % i != 0{
				break 'inner;
			}
			if i == 20{
				break 'outer;
			}
		}
		current_num += 20;
	}
    println!("Target Number: {}", current_num);
}
