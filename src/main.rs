// disable warnings only in dev mode
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

mod api;
mod app;
mod config;

#[macro_use]
extern crate log;
use crate::config::{APP_ID, LOCALE_DIR, PKG_NAME, PKG_VERSION};
use gettextrs::LocaleCategory;
use gtk::glib::{Continue, MainContext, PRIORITY_DEFAULT};
use gtk::prelude::*;
use gtk::Application;
use once_cell::sync::Lazy as SyncLazy; // std::lazy::SyncLazy in unstable rust
use std::env;
use tokio::runtime::{Builder as RuntimeBuilder, Runtime};

pub static RUNTIME: SyncLazy<Runtime> = SyncLazy::new(|| {
    RuntimeBuilder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap()
});

pub static API_CLIENT: SyncLazy<api::Client> =
    SyncLazy::new(|| api::Client::new("http://127.0.0.1:4567"));

fn main() {
    env_logger::init();
    // trace!("a trace example");
    // debug!("Version: {}", PKG_VERSION);
    // info!("such information");
    // warn!("o_O");
    // error!("boom");

    // let args: Vec<String> = env::args().collect();

    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(PKG_NAME, LOCALE_DIR).expect("Unable to bind the text domain");
    gettextrs::textdomain(PKG_NAME).expect("Unable to switch to the text domain");
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        let (tx1, rx) = MainContext::channel(PRIORITY_DEFAULT);
        let tx2 = tx1.clone();
        let mut model = app::Model::new(tx1);
        let mut view = app::View::new(tx2, app);
        rx.attach(None, move |msg| {
            // let msg = result.expect("application error: ");
            model.update(&msg);
            view.refresh(&msg, &model);
            Continue(true)
        });
    });
    app.run();
}
