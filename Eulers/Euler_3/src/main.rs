fn main() {
    let in_num: i64 = 600851475143*;
    let mut start = 3;
    let mut factors = Vec::new();
    let mut target_max: i64 = in_num;
    let mut is_prime: bool;

    while start <= target_max {
    	is_prime = true;
    	if in_num % start == 0 {
	    	for factor in &factors {
	    		if start % factor == 0 {
	    			is_prime = false;
	    			break;
	    		}
	    	}
	    	if is_prime == true{
	    		println!("Current Largest Prime Factor: {}",start);
	    		factors.push(start);
	    		target_max = in_num/start;
	    	}
	    }
    	start += 2;
    	
    }
    
}
