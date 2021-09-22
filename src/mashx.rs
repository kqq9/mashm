use std::{thread::sleep,time::Duration,process::{Command,Stdio}};
use gumdrop::Options;

#[derive(Options)]
struct MashmOptions {
	help: bool,
	#[options(meta="N")]
	length: Option<u16>,
	shout: bool,
}

fn main()
{
	let opts = MashmOptions::parse_args_default_or_exit();
	sleep(Duration::from_millis(500));
	let mut mash = Command::new("mashm");
	mash.arg("-n");
	if let Some(n) = opts.length {
		mash.arg(format!("-l{}", n));
	}
	if opts.shout {
		mash.arg("-s");
	}
	mash.stdout(Stdio::piped());
	let mash_handle = mash.spawn().unwrap();
	let mut typer = Command::new("xdotool");
	typer.args(["type", "--delay", "1", "--clearmodifiers", "--file", "-"]);
	typer.stdin(Stdio::from(mash_handle.stdout.unwrap()));
	typer.spawn().unwrap();
}
