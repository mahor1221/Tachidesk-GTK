use crate::api::*;
use crate::{API_CLIENT, RUNTIME};
use gtk::glib::Sender;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, HeaderBar, Label, Paned, Stack, StackSidebar};
use std::error::Error;

pub enum Message {
    SourceList(Option<Vec<Source>>),
}

pub struct Model {
    tx: Sender<Result<Message, Box<dyn Error + Sync + Send>>>,
    source_list: Option<Vec<Source>>,
}

impl Model {
    pub fn new(tx: Sender<Result<Message, Box<dyn Error + Sync + Send>>>) -> Self {
        let _tx = tx.clone();
        let handle = RUNTIME.spawn(async move {
            let result = API_CLIENT.get_source_list().await;
            let result = result.map(|option| Message::SourceList(option));
            _tx.send(result).expect("Receiver");
        });

        Self {
            tx,
            source_list: None,
        }
    }

    pub fn update(&mut self, msg: &mut Message) {
        match msg {
            Message::SourceList(option) => {
                self.source_list = option.take();
            }
        }
    }
}

pub struct View {
    tx: Sender<Result<Message, Box<dyn Error + Sync + Send>>>,
    sidebar_stack: Stack,
}

impl View {
    pub fn new(
        tx: Sender<Result<Message, Box<dyn Error + Sync + Send>>>,
        app: &Application,
    ) -> Self {
        let sidebar_stack = Stack::builder().build();
        let label = Label::builder().label("Test").build();
        sidebar_stack.add_titled(&label, None, &label.label().to_string());
        let sidebar = StackSidebar::builder().stack(&sidebar_stack).build();

        let paned = Paned::builder()
            .resize_start_child(false)
            .shrink_start_child(false)
            .start_child(&sidebar)
            .end_child(&sidebar_stack)
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

        Self { tx, sidebar_stack }
    }

    pub fn refresh(&mut self, msg: &Message, model: &Model) {
        match msg {
            Message::SourceList(_) => {
                // TODO: Handle None variant
                let source_list = model.source_list.as_ref().unwrap();
                for source in source_list {
                    if source.lang == "en" {
                        continue;
                    }
                    let label = Label::builder().label(&source.display_name).build();
                    self.sidebar_stack
                        .add_titled(&label, None, &label.label().to_string());
                }
            }
        }
    }
}
