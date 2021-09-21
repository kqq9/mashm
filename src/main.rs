use gumdrop::Options;

const KEYS: &str = "asdfghjkl;";

#[derive(Options)]
struct MashmOptions {
	help: bool,
	#[options(meta="N")]
	length: Option<u16>,
	// TODO: use upper row maybe
}

fn main() {
	let opts = MashmOptions::parse_args_default_or_exit();
	let mash_len = opts.length.unwrap_or(20);
	let mut s = [100; 10];
	let mut sum = 1000;
	for _ in 0..mash_len {
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
		if sum < 900 {
			s.iter_mut().for_each(|x| *x += 20);
			sum += 200;
		}
		print!("{}", &KEYS[keyi..keyi+1]);
	}
	println!();
}
