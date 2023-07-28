use common::get_supported_language_string;
use common::SupportedLanguageCode;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

pub mod common;

type LocalizedString = std::collections::HashMap<String, String>;

#[derive(Deserialize, Debug)]
pub struct MangaList {
    pub data: Vec<Manga>,
}

#[derive(Deserialize, Debug)]
pub struct Manga {
    pub id: Uuid,
    pub r#type: String,
    pub attributes: MangaAttributes,
    #[serde(default)]
    pub relationships: Vec<Relationship>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MangaAttributes {
    pub title: LocalizedString,
    pub alt_titles: Vec<LocalizedString>,
    pub description: LocalizedString,
    pub is_locked: bool,
}

#[derive(Deserialize, Debug)]
struct Tags {
    data: Vec<Tag>,
}

#[derive(Deserialize, Debug)]
struct Tag {
    id: Uuid,
    r#type: String,
    attributes: TagAttributes,
    relationships: Vec<Relationship>,
}

#[derive(Deserialize, Debug)]
struct TagAttributes {
    name: LocalizedString,
    description: LocalizedString,
    group: String,
    version: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    pub id: Uuid,
    pub r#type: String,
    #[serde(default)]
    // Always assume relationships are cover attributes since that is all we care about (very bad)
    pub attributes: CoverAttributes,
}

#[derive(Default, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CoverAttributes {
    pub file_name: String,
}

async fn send_request<T, U>(uri: &str, request_body: Option<U>) -> T
where
    T: DeserializeOwned,
    U: Serialize,
{
    let base_url = "https://api.mangadex.org";

    let full_url = format!("{}{}", base_url, uri);

    let client = reqwest::Client::new();
    let mut res = client.get(full_url);

    if let Some(body) = request_body {
        res = res.query(&body);
    }

    let res = res.send().await.unwrap();

    let out = &res.text().await.unwrap();

    let body_map: T = serde_json::from_str(&out).unwrap();

    body_map
}

pub async fn get_tag_ids(included_tags: &Vec<&str>) -> Vec<String> {
    let body_map: Tags = send_request("/manga/tag", None::<()>).await;

    let mut included_tag_ids: Vec<String> = Vec::new();

    for tag in body_map.data {
        let en_tag_name = tag
            .attributes
            .name
            .get(get_supported_language_string(
                SupportedLanguageCode::English,
            ))
            .unwrap();

        for included_tag in included_tags {
            if en_tag_name.to_uppercase() == included_tag.to_uppercase() {
                included_tag_ids.push(tag.id.to_string());
            }
        }
    }

    included_tag_ids
}

pub async fn get_manga(included_tag_ids: &Vec<String>, limit: i32, offset: i32) -> MangaList {
    let request_content = [
        ("includedTags[]", included_tag_ids.join(",")),
        ("includes[]", "cover_art".to_string()),
        ("limit", limit.to_string()),
        ("offset", offset.to_string()),
    ];

    let body_map: MangaList = send_request("/manga", Some(request_content)).await;
    body_map
}

pub async fn get_cover_art(manga_id: &Uuid, filename: &str) {
    let base_url = "https://uploads.mangadex.org";

    let full_url = format!("{}{}/{}/{}", base_url, "/covers", manga_id, filename);

    println!("{full_url}");

    let client = reqwest::Client::new();
    let res = client.get(full_url).send().await.unwrap();

    let bytes = &res.bytes().await.unwrap();

    let mut file = File::create(format!("{}/{}", "test-outputs/covers", filename)).unwrap();
    file.write_all(&bytes).unwrap();
}
