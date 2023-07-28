use mangafilter_prototype::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let included_tags = vec!["fantasy"];

    let included_tag_ids = get_tag_ids(&included_tags).await;

    let mangas = get_manga(&included_tag_ids, 2, 0).await;

    for manga in mangas.data {
        for relationship in manga.relationships {
            if relationship.r#type == "cover_art" {
                let filename = relationship.attributes.file_name;

                get_cover_art(&manga.id, &filename).await;
            }
        }
    }

    Ok(())
}
