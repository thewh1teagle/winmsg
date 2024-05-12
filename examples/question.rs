use winmsg::{message_box, Action, MessageBoxKind, Options};

fn main() {
    let result = message_box(Options {
        kind: MessageBoxKind::YesNoCancel,
        ..Default::default()
    });
    match result {
        Action::Yes => {
            println!("Yes");
        }
        Action::NO => {
            println!("No");
        }
        Action::Cancel => {
            println!("Cancel");
        }
        _ => {}
    }
}
