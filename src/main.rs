use std::{path::PathBuf, process};

use clap::Parser;
use thiserror::Error;
use url::Url;
use ytd_rs::{Arg, YoutubeDL};

#[derive(Error, Debug)]
enum YTDLError {
    #[error("Failed parsing url: {0}")]
    URLParseError(String),
}

type YTDLResult<T> = std::result::Result<T, YTDLError>;

#[derive(Debug, Parser)]
#[command(
    author = "Stoyko Tolev",
    version = "0.1.0",
    about = "An easy way to download YT videos"
)]
struct YTDLArguments {
    /// Filename to store locally. Defaults to some name.
    #[arg(long, short)]
    file_name: Option<String>,

    /// The directory where the file should be saved. It defaults to the current directory.
    #[arg(long, short)]
    directory: Option<String>,

    /// The url for the video that you'd like to download from youtube
    #[arg(short)]
    url: String,
}
struct URL {
    url: String,
}

impl URL {
    fn new(new_url: String) -> YTDLResult<URL> {
        let url = match Url::parse(&new_url) {
            Ok(parsed_url) => parsed_url.as_str().to_owned(),
            Err(err) => {
                return Err(YTDLError::URLParseError(err.to_string()));
            }
        };
        Ok(URL { url })
    }
}

struct YTDL {
    directory: PathBuf,
    arguments: Vec<Arg>,
    url: String,
}

impl YTDL {
    fn build(user_arguments: YTDLArguments) -> YTDL {
        let mut path = PathBuf::from("/Users/stoykotolev/Documents/youtube/");
        if let Some(directory) = user_arguments.directory {
            path.push(directory)
        };

        let file_name = format!(
            "{}.mp4",
            user_arguments.file_name.unwrap_or("video-lul".to_owned())
        );

        let args = vec![
            Arg::new("--progress"),
            Arg::new_with_arg("-f", "best"),
            Arg::new_with_arg("-o", &file_name),
        ];

        let url = match URL::new(user_arguments.url) {
            Ok(url) => url,
            Err(err) => {
                eprintln!("{:?}", err);
                process::exit(1);
            }
        };

        YTDL {
            directory: path,
            arguments: args,
            url: url.url,
        }
    }

    fn init_client(&self) -> YoutubeDL {
        let ytd = match YoutubeDL::new(&self.directory, self.arguments.clone(), &self.url) {
            Ok(ytd) => ytd,
            Err(err) => {
                eprintln!("Failed initializing download: {}", err.to_string());
                process::exit(1);
            }
        };

        eprintln!("Started download of video.");
        ytd
    }
}

fn main() {
    let user_args: YTDLArguments = YTDLArguments::parse();

    let ytdl = YTDL::build(user_args).init_client().download();

    let download = match ytdl {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Failed download: {}", err.to_string());
            process::exit(1);
        }
    };

    println!(
        "File downloaded at: {:?}",
        download.output_dir().to_string_lossy()
    );
}
