use winmsg::{message_box, Options};
fn main() {
    message_box(Options {
        title: "Hello world!".into(),
        description: "How are you?".into(),
        ..Default::default()
    });
}
