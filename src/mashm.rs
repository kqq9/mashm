use rand::random;
use gumdrop::Options;

const KEYS: &str = "asdfghjkl;";
// do people hold shift or use caps lock? that semicolon is confusing
const KEYS_SHOUTY: &str = "ASDFGHJKL;";

#[derive(Options)]
struct MashmOptions {
	help: bool,
	#[options(meta="N")]
	length: Option<u16>,
	#[options(short="n")]
	suppress_newline: bool,
	shout: bool,
}

fn main() {
	let opts = MashmOptions::parse_args_default_or_exit();
	let mash_len = opts.length.unwrap_or(20);
	let mut s = [100; 10]; // relative frequencies key by key
	let mut sum = 1000;
	let alphabet = if opts.shout { KEYS_SHOUTY } else { KEYS };
	for _ in 0..mash_len {
		let mut r = random::<i32>().rem_euclid(sum);
		let mut keyi = 9;
		for i in 0..10 {
			r -= s[i];
			if r < 0 {
				keyi = i;
				break;
			}
		}
		print!("{}", &alphabet[keyi..keyi+1]);
		// when a key is selected, distribute its probability of being
		// selected to the other keys
		let bump = s[keyi] / 9;
		for i in 0..10 {
			if i != keyi {
				s[i] += bump;
			}
		}
		sum -= s[keyi] % 9;
		s[keyi] = 0;
		// keep the total roughly constant
		if sum < 900 {
			s.iter_mut().for_each(|x| *x += 20);
			sum += 200;
		}
	}
	if mash_len > 0 && !opts.suppress_newline {
		println!();
	}
}
