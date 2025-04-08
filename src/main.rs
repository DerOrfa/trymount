use clap::Parser;
use clap::ValueHint::DirPath;
use std::time::Duration;
use tokio::fs::try_exists;
use tokio::time::timeout;

/// Experimental tool to trigger UNIX automounts or time out
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
	/// directory or file to access
	#[arg(value_hint = DirPath)]
	path:std::path::PathBuf,

	/// number of milliseconds before timeout is triggered
	#[arg(short, long, default_value_t = 1000)]
	timeout: u64,
}

#[tokio::main]
async fn main() -> Result<(),String>
{
	let args = Args::parse();
	let spath= args.path.to_string_lossy();
	match timeout(Duration::from_millis(args.timeout), try_exists(&args.path)).await
	{
		Ok(Ok(true)) => Ok(()), //didn't time out / could access
		Ok(Ok(false)) => Err(format!("{spath} does not exist")),
		Ok(Err(e)) => Err(format!("Accessing {spath} failed: {e}")),
		Err(_) => Err(format!("Accessing {spath} timed out")),
	}
}
