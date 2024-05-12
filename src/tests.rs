use crate::enums::*;
use crate::message_box;
use crate::Options;

#[test]
fn simple() {
    // simple message box
    message_box(Options::default());
}

#[test]
fn yes_no() {
    // yes no
    let ret = message_box(Options {
        kind: MessageBoxKind::YesNo,
        ..Default::default()
    });
    match ret {
        Action::Yes => println!("yes..."),
        _ => println!("no..."),
    }
}

#[test]
fn retry_cancel() {
    let ret = message_box(Options {
        kind: MessageBoxKind::RetryCancel,
        ..Default::default()
    });
    match ret {
        Action::Retry => println!("retry..."),
        _ => println!("cancel..."),
    }
}
