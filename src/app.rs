use gtk::glib::Sender;
use gtk::prelude::*;
use gtk::*;

pub enum Message {}

pub struct Model {}

impl Model {
    pub fn new(tx: Sender<Message>) -> Self {
        Self {}
    }

    pub fn update(&mut self, msg: &Message) {
        // match msg {
        // }
    }
}

pub struct View {}

impl View {
    pub fn new(tx: Sender<Message>, app: &Application) -> Self {
        let window = ApplicationWindowBuilder::new()
            .application(app)
            .title("Tachidesk")
            .default_width(1280)
            .default_height(720)
            .build();

        let header_bar = HeaderBarBuilder::new().build();

        window.set_titlebar(Some(&header_bar));
        // window.set_child(Some(&center_box));
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

        Self {}
    }

    pub fn refresh(&mut self, msg: &Message, model: &Model) {
        // match msg {
        // }
    }
}
