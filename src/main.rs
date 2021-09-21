const KEYS: &str = "asdfghjkl;"; // TODO: keys from other rows sometimes

fn main() {
	let mut s = [100; 10];
	let mut sum = 1000;
	for _ in 0..20 {
		let mut r = rand::random::<i32>().rem_euclid(sum);
		let mut keyi = 9;
		for i in 0..10 {
			r -= s[i];
			if r < 0 {
				keyi = i;
				break;
			}
		}
		let bump = s[keyi] / 9;
		for i in 0..10 {
			if i != keyi {
				s[i] += bump;
			}
		}
		sum -= s[keyi] % 9;
		s[keyi] = 0;
		print!("{}", &KEYS[keyi..keyi+1]);
	}
	println!();
}
