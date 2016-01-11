static MAP : [char; 41]= [' ', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
                          'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '#',
						  '.', ',', '\'', '!', '?', '(', ')', '-', ':', '$', '/', '&', '\\'];

pub fn decrypt(st: &str) -> Result<String, u64> {
	let tmp = step1(st);
	let tmp = try!(step2(tmp));
	let tmp = step3(tmp);
	let tmp = step4(tmp);
	Ok(tmp)
}

fn step1(st: &str) -> Vec<char> {
	let mut res = Vec::new();
	let mut i = 0;
	for c in st.chars() {
		if (i + 1) % 8 != 0 {
			res.push(c);
		}
		i += 1;
	}
	res
}

fn step2(st: Vec<char>) -> Result<Vec<u64>, u64> {
	if st.len() % 6 != 0 {
		// incorrect length
		return Err(1);
	} else {
		let mut res = Vec::new();
		let mut fred = [0; 6]; // this holds six values
		let mut i = 0; // index of fred
        let base: u64 = 41;
		for ch in &st {
			if let Some(c) = find(*ch) {
				fred[i] = c;
				i += 1;
				if i > 5 {
					// make into one number, insert into res
                    let num = base.pow(5)*fred[0] + base.pow(4)*fred[1] + base.pow(3)*fred[2]
                    + base.pow(2)*fred[3] + base*fred[4] + fred[5];
					res.push(num);
                    i = 0;
				}
			} else {
				// unknown chracter
				return Err(2);
			}
		}
        Ok(res)
	}
}

fn find(ch: char) -> Option<u64> {
	let mut i = 0;
	while i < 41 {
		if MAP[i] == ch {
			return Some(i as u64);
		}
        i += 1;
	}
	// i == 41; no match
	None
}

fn step3(nums: Vec<u64>) -> Vec<u64> {
    let mut res = Vec::new();
    for num in nums {
        res.push(mod_exp(num));
    }
    res
}

fn mod_exp(num: u64) -> u64 {
    let mut m = 1;
    let n = 4294434817;
    let mut d = 1921821779;
    let mut c = num % n;
    while d > 0 {
        if d % 2 == 1 {
            m = (m * c) % n;
        }
        d /= 2;
        c = (c * c) % n;
    }
    m
}

fn step4(nums: Vec<u64>) -> String {
    let mut res = String::new();
    let mut nums = nums;
    let mut temp = [0; 6];
    let mut i = 0;
    while i < nums.len() {
        let mut j = 6;
        while j > 0 {
            j -= 1;
            temp[j] = nums[i] % 41;
            nums[i] /= 41;
        }
        while j < 6 {
            res.push(MAP[temp[j] as usize]);
            j += 1;
        }
        i += 1;
    }
    res
}
