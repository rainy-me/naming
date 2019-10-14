use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TreeResponse {
    pub sha: String,
    pub url: String,
    pub tree: Vec<File>,
}

#[derive(Deserialize, Debug)]
pub struct File {
    pub path: String,
    pub mode: String,
    pub r#type: String,
    pub sha: String,
    pub url: String,
    pub size: Option<u64>,
}

pub fn get_tree(owner_and_repo: &str) -> Result<TreeResponse, reqwest::Error> {
    let url = [&get_sha(owner_and_repo).unwrap(), "?recursive=1"].concat();
    println!("sha: {}", url);
    let resp: TreeResponse = reqwest::get(&url)?.json()?;
    Ok(resp)
}

#[derive(Deserialize, Debug)]
pub struct ShaResponseTreeData {
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct ShaResponseTree {
    tree: ShaResponseTreeData,
}

#[derive(Deserialize, Debug)]
pub struct ShaResponseCommit {
    commit: ShaResponseTree,
}

#[derive(Deserialize, Debug)]
pub struct ShaResponse {
    commit: ShaResponseCommit,
}

pub fn get_sha(owner_and_repo: &str) -> Result<String, reqwest::Error> {
    let url = [
        "https://api.github.com/repos/",
        owner_and_repo,
        "/branches/master",
    ]
    .concat();

    let resp: ShaResponse = reqwest::get(&url)?.json()?;
    Ok(resp.commit.commit.tree.url)
}
