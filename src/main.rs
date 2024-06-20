use clap::{value_parser, Arg, Command};
use regex::Regex;

mod aria2;
mod hf;

#[derive(Clone)]
struct Model {
    organization: String,
    name: String,
    branch: String,
    path: String,
}

struct DownloadRequest {
    model: Model,
    aria2_options: aria2::Aria2Options,
}

fn main() {
    let request = parse_download_request();
    println!(
        "Downloading {}/{}:{} to {}",
        request.model.organization, request.model.name, request.model.branch, request.aria2_options.destination
    );

    let to_download = hf::get_list_of_files_to_download(&request.model);

    aria2::download_model(&request.aria2_options, to_download);
}

fn parse_download_request() -> DownloadRequest {
    let matches = Command::new("hf-aria-dl")
        .version("0.1.0")
        .author("Yurii Polishchuk <yuriipolishchuk1@gmail.com>")
        .about("Quickly download models from Hugging Face Hub - powered by aria2")
        .arg(
            Arg::new("model")
                .index(1)
                .help("Specifies the model in the format 'organization/model:branch' (default branch is 'main')")
                .required(true)
                .value_name("MODEL")
                .value_parser(|v: &str| -> Result<Model, &str> {
                    let model_re = Regex::new(
                        r"^(?P<organization>[^/\s/]+)/(?P<model>[^:\s/]+)(?::(?P<branch>[^\s/]+))?(?:/(?P<path>[^\s]+)?)?$",
                    )
                    .unwrap();
                    let captures = model_re.captures(&v).ok_or("Invalid model name format")?;
                    Ok(Model {
                        organization: captures.name("organization").unwrap().as_str().to_string(),
                        name: captures.name("model").unwrap().as_str().to_string(),
                        branch: captures
                            .name("branch")
                            .map(|m| m.as_str())
                            .unwrap_or("main")
                            .to_string(),
                        path: captures
                            .name("path")
                            .map(|m| m.as_str())
                            .unwrap_or("")
                            .to_string(),
                    })
                }),
        )
        .arg(
            Arg::new("destination")
                .index(2)
                .help("Sets the destination path")
                .required(true)
                .value_name("DESTINATION")
                .value_parser(value_parser!(String)),
        )
        .arg(
            Arg::new("par_files")
                .short('j')
                .long("max-concurrent-downloads")
                .help("Sets the number of parallel files to download")
                .value_name("PAR_FILES")
                .default_value("3")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            Arg::new("streams")
                .short('x')
                .long("max-connection-per-server")
                .help("Sets the number of streams per file download")
                .value_name("STREAMS")
                .default_value("5")
                .value_parser(value_parser!(u32)),
        )
        .get_matches();

    let model = matches.get_one::<Model>("model").unwrap().to_owned();
    let destination = {
        let arg = matches.get_one::<String>("destination").unwrap();
        // Create download directory from model name in the destination path
        // Format: <destination>/<organization>_<model>_<branch>/<path>
        format!("{}/{}_{}_{}/{}", arg, model.organization, model.name, model.branch, model.path)
    };

    DownloadRequest {
        model,
        aria2_options: aria2::Aria2Options {
            destination,
            par_files: matches.get_one::<u32>("par_files").unwrap().to_owned(),
            streams: matches.get_one::<u32>("streams").unwrap().to_owned(),
        },
    }
}