use crate::{api, API_CLIENT, RUNTIME};
use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio;
use gtk::glib::{Bytes, Sender};
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, FlowBox, FlowBoxChild, Frame, HeaderBar, Image, Label, Paned,
    ScrolledWindow, SelectionMode, Spinner, Stack, StackSidebar,
};
use std::error::Error;

// TODO: replace expect("...")

pub struct Manga {
    manga: api::Manga,
    thumbnail: Option<Bytes>,
}

impl std::ops::Deref for Manga {
    type Target = api::Manga;
    fn deref(&self) -> &Self::Target {
        &self.manga
    }
}

pub enum Message {
    SourceList(Option<Vec<api::Source>>),
    MangaList(Option<Vec<api::Manga>>),
    Thumbnail(Option<Bytes>, usize),
    Sidebar(u16),
}

pub struct Model {
    tx: Sender<Result<Message, Box<dyn Error + Sync + Send>>>,
    source_list: Option<Vec<api::Source>>,
    manga_list: Option<Vec<Manga>>,
}

impl Model {
    pub fn new(tx: Sender<Result<Message, Box<dyn Error + Sync + Send>>>) -> Self {
        let _tx = tx.clone();
        RUNTIME.spawn(async move {
            let result = API_CLIENT.get_source_list().await;
            let result = result.map(|option| Message::SourceList(option));
            _tx.send(result).expect("Receiver");
        });

        Self {
            tx,
            source_list: None,
            manga_list: None,
        }
    }
    pub fn update(&mut self, msg: &mut Message) {
        match msg {
            Message::SourceList(option) => {
                self.source_list = option.take();
            }
            Message::MangaList(option) => {
                // self.manga_list = option.take().map(|vec| {
                //     vec.into_iter()
                //         .map(|manga| Manga {
                //             manga,
                //             thumbnail: None,
                //         })
                //         .collect()
                // });
            }
            Message::Thumbnail(option, index) => {
                // self.manga_list.as_mut().expect("MangaList is None")[*index].thumbnail =
                //     option.take();
            }
            Message::Sidebar(_) => {}
        }
    }
    pub fn source_list(&self) -> Option<&Vec<api::Source>> {
        self.source_list.as_ref()
    }
    pub fn manga_list(&self) -> Option<&Vec<Manga>> {
        self.manga_list.as_ref()
    }
}

pub struct View {
    tx: Sender<Result<Message, Box<dyn Error + Sync + Send>>>,
    sidebar_stack: Stack,
    flowbox: FlowBox,
    manga_index: usize,
}

impl View {
    pub fn new(
        tx: Sender<Result<Message, Box<dyn Error + Sync + Send>>>,
        app: &Application,
    ) -> Self {
        let flowbox = FlowBox::builder()
            .selection_mode(SelectionMode::None)
            .build();
        let frame = Frame::builder().child(&flowbox).build();
        let scrolled_window = ScrolledWindow::builder().child(&frame).build();

        let sidebar_stack = Stack::builder().build();
        let sidebar = StackSidebar::builder().stack(&sidebar_stack).build();

        let paned = Paned::builder()
            .resize_start_child(false)
            .shrink_start_child(false)
            .start_child(&sidebar)
            .end_child(&scrolled_window)
            .build();

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Tachidesk")
            .default_width(1280)
            .default_height(720)
            .child(&paned)
            .build();
        let header_bar = HeaderBar::builder().build();
        window.set_titlebar(Some(&header_bar));
        window.show();

        let _tx = tx.clone();
        sidebar_stack.connect_visible_child_notify(move |stack| {
            let _tx = _tx.clone();
            let source_id = stack
                .visible_child_name()
                .expect("Visible StackChild is None");
            RUNTIME.spawn(async move {
                let result = API_CLIENT.get_manga_list(&source_id, 1).await;
                let result = result.map(|option| Message::MangaList(option));
                _tx.send(result).expect("Receiver");
            });
        });

        // let _tx = tx.clone();
        // let (counter1_tx, counter1_rx) = MainContext::channel(PRIORITY_DEFAULT);
        // self.counter1.transmit(counter1_tx);
        // counter1_rx.attach(None, move |msg| {
        //     _tx.send(Message::Counter1(msg)).unwrap();
        //     Continue(true)
        // });

        // let _tx = tx.clone();
        // let (counter2_tx, counter2_rx) = MainContext::channel(PRIORITY_DEFAULT);
        // self.counter2.transmit(counter2_tx);
        // counter2_rx.attach(None, move |msg| {
        //     _tx.send(Message::Counter2(msg)).unwrap();
        //     Continue(true)
        // });

        Self {
            tx,
            sidebar_stack,
            flowbox,
            manga_index: 0,
        }
    }

    pub fn refresh(&mut self, msg: &Message, model: &Model) {
        match msg {
            Message::SourceList(_) => {
                // TODO: Handle None variant
                let list = model.source_list().expect("SourceList is None");
                for source in list {
                    if source.lang != "en" {
                        continue;
                    }
                    let name = &source.display_name;
                    let label = Label::builder().label(name).build();
                    self.sidebar_stack
                        .add_titled(&label, Some(&source.id), name);
                }
            }
            Message::MangaList(option) => {
                // TODO: Handle None variant
                // let list = model.manga_list().expect("MangaList is None");
                let list = option.as_ref().expect("MangaList is None");
                self.flowbox = FlowBox::builder()
                    .selection_mode(SelectionMode::None)
                    .build();
                for manga in list {
                    let spinner = Spinner::builder()
                        .height_request(200)
                        .width_request(200)
                        .spinning(true)
                        .build();
                    // -1 means insert at the end
                    self.flowbox.insert(&spinner, -1);

                    // gtk::Image doesn't implement Send trait
                    // So we have to send glib::Bytes
                    let id = manga.id;
                    let index = self.manga_index;
                    let _tx = self.tx.clone();
                    RUNTIME.spawn(async move {
                        let result = API_CLIENT.get_manga_thumbnail(id).await;
                        let result = result.map(|option| Message::Thumbnail(option, index));
                        _tx.send(result).expect("Receiver");
                    });

                    self.manga_index += 1;
                }
            }
            Message::Thumbnail(option, index) => {
                // TODO: Handle None variant
                // let bytes = &model.manga_list().expect("MangaList is None")[*index]
                //     .thumbnail
                //     .as_ref()
                //     .expect("Thumbnail is None");
                let bytes = option.as_ref().expect("Thumbnail is None");
                let stream = gio::MemoryInputStream::from_bytes(&bytes);
                let pixbuf = Pixbuf::from_stream(&stream, Some(&gio::Cancellable::new()))
                    .expect("Can't create Pixbuf from stream");
                let image = Image::builder()
                    .height_request(200)
                    .width_request(200)
                    .build();
                image.set_from_pixbuf(Some(&pixbuf));

                self.flowbox
                    .child_at_index(*index as i32)
                    .expect("FlowboxChild is None at given index")
                    .set_child(Some(&image));
            }
            Message::Sidebar(_) => {}
        }
    }
}
