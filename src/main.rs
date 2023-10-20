use std::{path::PathBuf, process};

use clap::Parser;
use url::{ParseError, Url};
use ytd_rs::{Arg, YoutubeDL};

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
    url: Result<String, ParseError>,
}

impl URL {
    fn new(new_url: String) -> Self {
        let url = match Url::parse(&new_url) {
            Ok(parsed_url) => Ok(parsed_url.as_str().to_owned()),
            Err(err) => Err(err),
        };
        Self { url }
    }
}

fn main() {
    let user_args: YTDLArguments = YTDLArguments::parse();
    let path = PathBuf::from("./download");
    let file_name = format!(
        "{}.mp4",
        user_args.file_name.unwrap_or("video-lul".to_owned())
    );
    let url = match URL::new(user_args.url).url {
        Ok(url) => url,
        Err(err) => {
            eprintln!("Failed parsing url: {:?}", err.to_string());
            process::exit(0)
        }
    };

    let args = vec![
        Arg::new("--progress"),
        Arg::new_with_arg("-f", "best"),
        Arg::new_with_arg("-o", &file_name),
    ];

    eprintln!("Started download of video.");
    let ytd = match YoutubeDL::new(&path, args, &url) {
        Ok(ytd) => ytd,
        Err(err) => {
            eprintln!("fek. {:?}", err);
            return;
        }
    };

    let download = match ytd.download() {
        Ok(resu) => resu,
        Err(err) => {
            eprintln!("fek download. {:?}", err);
            return;
        }
    };

    println!(
        "File downloaded at: {:?}",
        download.output_dir().to_string_lossy()
    );
}
