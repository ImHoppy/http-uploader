use std::thread;
use std::time::{Duration, SystemTime};
use std::fs;
use std::path::PathBuf;
use std::fs::File;

use log::{info, debug, error};

use crate::Args;

pub fn create_schedule_thread(args: Args) {
	thread::spawn(move || {
		loop {
			debug!("Checking for files to delete");
			let files = get_files(args.upload_dir.clone());
			let mut cleaned_files = 0;

			for (file, path) in files {
				match delete_file(file, &path, args.file_lifetime) {
					Ok(true) => cleaned_files += 1,
					Ok(false) => (),
					Err(err) => error!("Failed to delete file: {}", err),
				}
			}
			info!("Clean up routine, {} files deleted", cleaned_files);

			thread::sleep(Duration::from_secs(24 * 60 * 60));
		}
	});
}

fn get_files(upload_dir: String) -> Vec<(File, PathBuf)> {
	let mut files: Vec<(File, PathBuf)> = Vec::new();

	if let Ok(entries) = fs::read_dir(upload_dir) {
		for entry in entries {
			if let Ok(entry) = entry {
				let path = entry.path();
				let filename = entry.file_name();

				// Assume that the filename is in the format of UUID.EXTENSION
				if path.is_file() && filename.len() > 35 && filename.to_str().unwrap().matches('-').count() == 4 {
					debug!("Found file: {:?}", path);
					if let Ok(file) = File::open(&path) {
						files.push((file, path));
					}
				} else if path.is_dir() {
					let sub_files = get_files(path.to_string_lossy().into_owned());
					files.extend(sub_files);
				}
			}
		}
	}

	files
}

fn format_duration(d: Duration) -> String {
	let secs = d.as_secs();
	let mins = secs / 60;
	let hours = mins / 60;
	let days = hours / 24;

	format!("{}d {}h {}m {}s", days, hours % 24, mins % 60, secs % 60)
}

fn delete_file(file: File, path: &PathBuf, file_lifetime: u64) -> Result<bool, std::io::Error> {
	let access_date = file.metadata()?.accessed()?;

	let difference = SystemTime::now().duration_since(access_date).unwrap();
	let needs_deletion = file_lifetime > 0 && difference > Duration::from_secs(file_lifetime);
	if needs_deletion {
		fs::remove_file(path)?;
		debug!("{:?} deleted, inactivity for {}", path, format_duration(difference));
	}
	Ok(needs_deletion)
}
