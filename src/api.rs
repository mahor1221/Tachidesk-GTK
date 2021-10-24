use gtk::glib::Bytes;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Extension {
    pub apk_name: String,
    pub icon_url: String,
    pub name: String,
    pub pkg_name: String,
    pub version_name: String,
    pub version_code: u8,
    pub lang: String,
    pub is_nsfw: bool,
    pub installed: bool,
    pub has_update: bool,
    pub obsolete: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub id: String,
    pub name: String,
    pub lang: String,
    pub icon_url: String,
    pub supports_latest: bool,
    pub is_configurable: bool,
    pub is_nsfw: bool,
    pub display_name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Manga {
    pub id: u16,
    pub source_id: String,
    pub url: String,
    pub title: String,
    pub thumbnail_url: String,
    pub initialized: bool,
    pub artist: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub genre: Vec<String>,
    pub status: String, // enum
    pub in_library: bool,
    pub in_library_at: u32,
    pub source: Option<Source>,
    // pub meta: {},
    pub real_url: Option<String>,
    pub fresh_data: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MangaList {
    pub manga_list: Vec<Manga>,
}

// TODO: use strum crate
// pub enum MangaStatus {
//     Unknown,
//     Ongoing,
//     Completed,
//     Dropped,
// }

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Chapter {
    pub url: String,
    pub name: String,
    pub upload_date: u64,
    pub chapter_number: f32,
    pub scanlator: String,
    pub manga_id: u16,
    pub read: bool,
    pub bookmarked: bool,
    pub last_page_read: u16,
    pub last_read_at: u32,
    pub index: u16,
    pub fetched_at: u64,
    pub downloaded: bool,
    pub page_count: i8,
    pub chapter_count: u16,
    // pub meta: {}
}

// pub fn async_wraper<T>(
//     runtime: Runtime,
//     sender: Sender<Message>,
//     async_fn: &dyn Fn(Query) -> Result<Option<T>, Error>,
// ) {
//     RUNTIME.spawn({
//         response = async_fn(Query);
//         let msg = Message::SourceList(T);
//         sender.send(msg).unwrap();
//     });
// }

// Function from https://gitlab.gnome.org/GNOME/fractal/-/blob/fractal-next/src/utils.rs
// pub fn do_async<
//     R: Send + 'static,
//     F1: Future<Output = R> + Send + 'static,
//     F2: Future<Output = ()> + 'static,
//     FN: FnOnce(R) -> F2 + 'static,
// >(
//     priority: glib::source::Priority,
//     tokio_fut: F1,
//     glib_closure: FN,
// ) {
//     let (sender, receiver) = tokio::sync::oneshot::channel();

//     glib::MainContext::default().spawn_local_with_priority(priority, async move {
//         glib_closure(receiver.await.unwrap()).await
//     });

//     RUNTIME.spawn(async move { sender.send(tokio_fut.await) });
// }

// TODO: handle emplty responses
// TODO: handle errors when tachidesk-server is not running

#[derive(Clone)]
pub struct Client {
    // server: Url,
    server: &'static str,
}

impl Client {
    pub fn new(server: &'static str) -> Self {
        Self { server }
    }

    pub async fn get_source_list(
        &self,
    ) -> Result<Option<Vec<Source>>, Box<dyn Error + Sync + Send>> {
        let url = format!("{}/api/v1/source/list", self.server);
        let response = reqwest::get(url).await?.json::<Vec<Source>>().await?;
        Ok(Some(response))
    }

    pub async fn get_manga_list(
        &self,
        source_id: &'static str,
        page: u16,
    ) -> Result<Option<Vec<Manga>>, Box<dyn Error + Sync + Send>> {
        let url = format!(
            "{}/api/v1/source/{}/latest/{}",
            self.server, source_id, page
        );
        let response = reqwest::get(url).await?.json::<MangaList>().await?;
        Ok(Some(response.manga_list))
    }

    pub async fn get_manga(
        &self,
        manga_id: u16,
    ) -> Result<Option<Manga>, Box<dyn Error + Sync + Send>> {
        let url = format!("{}/api/v1/manga/{}", self.server, manga_id);
        let response = reqwest::get(url).await?.json::<Manga>().await?;
        Ok(Some(response))
    }

    pub async fn get_manga_thumbnail(
        &self,
        manga_id: u16,
    ) -> Result<Option<Bytes>, Box<dyn Error + Sync + Send>> {
        let url = format!("{}/api/v1/manga/{}/thumbnail", self.server, manga_id);
        let response = reqwest::get(url).await?.bytes().await?;
        Ok(Some(Bytes::from_owned(response)))
    }

    pub async fn get_manga_chapter_list(
        &self,
        manga_id: u16,
    ) -> Result<Option<Vec<Chapter>>, Box<dyn Error + Sync + Send>> {
        let url = format!("{}/api/v1/manga/{}/chapters", self.server, manga_id);
        let response = reqwest::get(url).await?.json::<Vec<Chapter>>().await?;
        Ok(Some(response))
    }

    pub async fn get_manga_page(
        &self,
        manga_id: u16,
        chapter: u16,
        page: u16,
    ) -> Result<Option<Bytes>, Box<dyn Error + Sync + Send>> {
        let url = format!(
            "{}/api/v1/manga/{}/chapter/{}/page/{}",
            self.server, manga_id, chapter, page
        );
        let response = reqwest::get(url).await?.bytes().await?;
        Ok(Some(Bytes::from_owned(response)))
    }
}
