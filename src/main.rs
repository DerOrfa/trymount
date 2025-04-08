use clap::Parser;
use clap::ValueHint::DirPath;
use std::path::PathBuf;
use std::process::exit;
use std::time::Duration;
use tokio::fs::try_exists;
use tokio::time::timeout;

/// Experimental tool trigger UNIX automounts or time out
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	/// directory or file to access
	#[arg(value_hint = DirPath)]
	path:PathBuf,

	/// number of milliseconds before timeout is triggered
	#[arg(short, long, default_value_t = 1000)]
	timeout: u64,
}

#[tokio::main]
async fn main()
{
	let args = Args::parse();
	let spath= args.path.to_string_lossy();
	match timeout(Duration::from_millis(args.timeout), try_exists(&args.path)).await
	{
		Ok(Ok(true)) => exit(0), //didn't time out / could access
		Ok(Ok(false)) => println!("{spath} does not exist",),
		Ok(Err(e)) => println!("Accessing {spath} failed: {e}"),
		Err(_) => println!("Accessing {spath} timed out"),
	}
	exit(-1);
}
