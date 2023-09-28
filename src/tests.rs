

mod tests {
    use crate::enums::*;
    use crate::message_box;
    
    #[test]
    fn simple() {
        // simple message box
        message_box(Some("Error"), Some("something happend"), None, None, None);
    }

    #[test]
    fn yes_no() {
        
        // yes no
        let ret = message_box(Some("Error"), Some("something happend"), Some(MessageBoxType::YesNo), None, None);
        match ret {
            MessageBoxReturnCode::YES => println!("yes..."),
            _ => println!("no...")
        }
    }

    #[test]
    fn retry_cancel() {
        let ret = message_box(Some("Error"), Some("something happend"), Some(MessageBoxType::RetryCancel), None, None);
        match ret {
            MessageBoxReturnCode::RETRY => println!("retry..."),
            _ => println!("cancel...")
        }
    }
}
