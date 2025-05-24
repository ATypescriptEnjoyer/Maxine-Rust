use poise::{serenity_prelude as serenity, CreateReply};
use std::path::PathBuf;
use tokio::fs;
use std::process::Command;
use tempfile::NamedTempFile;

use crate::structs::Data;

const IGNORE_USER_AGENT_HOSTS: [&str; 1] = ["reddit.com"];

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

fn format_timestamp(ts: &str) -> String {
	ts.split(':')
		.map(|val| format!("{:0>2}", val))
		.collect::<Vec<_>>()
		.join(":")
}

/// Saves video from URL
#[poise::command(slash_command, prefix_command)]
pub async fn save(
	ctx: Context<'_>,
	#[description = "Video URL"] url: String,
	#[description = "Start of clip (HH:MM:SS)"] clip_start: Option<String>,
	#[description = "End of clip (HH:MM:SS)"] clip_end: Option<String>,
	#[description = "Custom file format (gif, webm) default is MP4"] as_format: Option<String>,
) -> Result<(), Error> {
	ctx.defer().await?;

	let is_clip = clip_start.is_some() && clip_end.is_some();
	let format = as_format.unwrap_or_else(|| "mp4".to_string());

	// Download video
	let file_path = match download_video(&url, false).await {
		Ok(path) => path,
		Err(err) => {
			ctx.say(format!("Error downloading video: {}", err)).await?;
			return Ok(());
		}
	};

	let first_file_path = file_path.clone();
	let file_path_ext = file_path
		.extension()
		.and_then(|ext| ext.to_str())
		.unwrap_or("mp4");
	let ext = if file_path_ext == format { file_path_ext } else { &format };

	// Process video if needed
	let final_path = if is_clip {
		let ffmpeg_command = format!(
			"-ss {} -to {}",
			format_timestamp(clip_start.as_ref().unwrap()),
			format_timestamp(clip_end.as_ref().unwrap())
		);
		
		match convert_file(&file_path, ext, Some(&ffmpeg_command)).await {
			Ok((_, _, file)) => file,
			Err(err) => {
				ctx.say(format!("Error converting video: {}", err)).await?;
				return Ok(());
			}
		}
	} else {
		file_path
	};

	// Send the file
	let file = serenity::CreateAttachment::path(&final_path).await?;
	let reply = CreateReply::default().attachment(file);

	ctx.send(reply).await?;

	// Clean up temporary files
	let _ = fs::remove_file(&final_path).await;
	if first_file_path != final_path {
		let _ = fs::remove_file(&first_file_path).await;
	}

	Ok(())
}

async fn download_video(url: &str, show_warnings: bool) -> Result<PathBuf, Error> {
	let temp_file = NamedTempFile::new()?;
	let output_path = temp_file.path().with_extension("mp4");
	
	let mut cmd = Command::new("yt-dlp");
	cmd.arg(url)
		.arg("-o")
		.arg(output_path.to_str().unwrap())
		.arg("-f")
		.arg("bv*[ext=mp4][vcodec=h264]+ba[ext=m4a]/b[ext=mp4][vcodec=h264]/bv[vcodec=h264]+ba/bv+ba/b")
		.arg("--compat-opt")
		.arg("prefer-vp9-sort")
		.arg("--no-check-certificate");

	if !IGNORE_USER_AGENT_HOSTS.iter().any(|host| url.contains(host)) {
		cmd.arg("--add-header")
			.arg("User-Agent:facebookexternalhit/1.1");
	}

	if !show_warnings {
		cmd.arg("--no-warnings");
	}

	let output = cmd.output()?;

  println!("{}", String::from_utf8_lossy(&output.stdout));
	
	if !output.status.success() || !output_path.try_exists().unwrap_or(false) {
		let error = String::from_utf8_lossy(&output.stderr);
		return Err(format!("Failed to download video: {}", error).into());
	}

	Ok(output_path)
}

async fn convert_file(
	input_path: &PathBuf,
	format: &str,
	ffmpeg_args: Option<&str>,
) -> Result<(i32, String, PathBuf), Error> {
	let temp_file = NamedTempFile::new()?;
	let output_path = temp_file.path().with_extension(format);
	
	let mut cmd = Command::new("ffmpeg");
	
	// Add input file
	cmd.arg("-i")
		.arg(input_path.to_str().unwrap());
	
	// Add any additional ffmpeg arguments (like -ss and -to for clips)
	if let Some(args) = ffmpeg_args {
		cmd.args(args.split_whitespace());
	}
	
	// Add output options
	cmd.arg("-c:v")
		.arg(if format == "gif" { "gif" } else { "libx264" })
		.arg("-preset")
		.arg("medium")
		.arg("-y") // Overwrite output file if it exists
		.arg(output_path.to_str().unwrap());
	
	let output = cmd.output()?;
	
	if !output.status.success() {
		let error = String::from_utf8_lossy(&output.stderr);
		return Err(format!("Failed to convert video: {}", error).into());
	}
	
	Ok((output.status.code().unwrap_or(1), 
		String::from_utf8_lossy(&output.stdout).to_string(),
		output_path))
}
