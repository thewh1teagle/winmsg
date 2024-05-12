

mod tests {
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
        let ret = message_box(Options {kind: MessageBoxType::YesNo, ..Default::default()});
        match ret {
            MessageBoxReturnCode::YES => println!("yes..."),
            _ => println!("no...")
        }
    }

    #[test]
    fn retry_cancel() {
        let ret = message_box(Options {kind: MessageBoxType::RetryCancel, ..Default::default()});
        match ret {
            MessageBoxReturnCode::RETRY => println!("retry..."),
            _ => println!("cancel...")
        }
    }
}
