use serde::Deserialize;

use crate::Model;
pub struct ToDownload {
    pub size: u64,
    pub path: String,
    pub url: String,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum ResourceKind {
    #[serde(rename = "file")]
    File { size: u64, path: String },
    #[serde(rename = "directory")]
    Directory { path: String },
}

pub fn get_list_of_files_to_download(model: &Model) -> Vec<ToDownload> {
    let mut files = Vec::new();
    let mut dirs_to_fetch = vec![model.path.clone()];

    while let Some(dir) = dirs_to_fetch.pop() {
        println!("Fetching file tree for /{dir}");
        let resp = ureq::get(&format!(
            "https://huggingface.co/api/models/{}/{}/tree/{}/{}",
            model.organization, model.name, model.branch, dir
        ))
        .call()
        .expect("Failed to get model info")
        .into_json::<Vec<ResourceKind>>()
        .expect("Failed to parse response");

        for item in resp {
            match item {
                ResourceKind::File { path, size } => {
                    files.push(ToDownload {
                        url: get_download_url(&model, &path),
                        size,
                        path,
                    });
                }
                ResourceKind::Directory { path } => {
                    dirs_to_fetch.push(path);
                }
            };
        }
    }

    files
}

fn get_download_url(model: &Model, path: &str) -> String {
    format!(
        "https://huggingface.co/{}/{}/resolve/{}/{}",
        model.organization, model.name, model.branch, path
    )
}
