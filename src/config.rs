use std::env;
use std::error::Error;
use std::fmt;
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;

#[derive(Debug)]
struct NoHomeDir;

impl Error for NoHomeDir {
	fn description(&self) -> &str { "Couldn't find home directory" }
}
impl fmt::Display for NoHomeDir {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { f.write_str(self.description()) }
}

pub fn make_configs() -> Result<(String, String, String), Box<Error>> {
	let mut folder = get_config_folder()?;
	folder.push("insult");
	fs::create_dir_all(&folder)?;

	let mut contents = (String::new(), String::new(), String::new());
	macro_rules! create_file {
		($name:expr, $index:tt) => {
			let path = folder.join($name);
			if path.exists() {
				let file = File::open(path)?;
				let mut reader = BufReader::new(file);
				reader.read_to_string(&mut contents.$index)?;
			} else {
				let mut file = File::create(path)?;
				file.write_all(include_bytes!($name))?;
				contents.$index = include_str!($name).to_string();
			}
		}
	}
	create_file!("nouns", 0);
	create_file!("endings", 1);
	create_file!("verbs", 2);

	Ok(contents)
}
fn get_config_folder() -> Result<PathBuf, Box<Error>> {
	if cfg!(target_os = "linux") {
		let xdg_config_home = env::var("XDG_CONFIG_HOME");
		if let Ok(dir) = xdg_config_home {
			Ok(PathBuf::from(dir))
		} else {
			let home = env::home_dir();
			match home {
				Some(mut home) => {
					home.push(".config");
					Ok(home)
				},
				None => Err(Box::new(NoHomeDir)),
			}
		}
	} else if cfg!(target_os = "macos") {
		let home = env::home_dir();
		match home {
			Some(mut home) => {
				home.push("Library");
				home.push("Preferences");
				Ok(home)
			},
			None => Err(Box::new(NoHomeDir)),
		}
	} else {
		Ok(env::current_dir()?)
	}
}
