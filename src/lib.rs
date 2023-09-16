use winapi::{shared::windef::HWND, um::winuser as user32};


pub enum MessageBoxIconType {
    WARNING = 0x00000030,
    INFORMATION = 0x00000040,
    QUESTION = 0x00000020,
    ERROR = 0x00000010,
}

pub enum MessageBoxType {
    AbortRetryIgnore = 0x00000002,
    CancelTryContinue = 0x00000006,
    HELP = 0x00004000,
    OK = 0x00000000,
    OkCancel = 0x00000001,
    RetryCancel = 0x00000005,
    YesNo = 0x00000004,
    YesNoCancel = 0x00000003,
}

pub enum MessageBoxFlags {
    DEFBUTTON1 = 0x00000000,
    DEFBUTTON2 = 0x00000100,
    DEFBUTTON3 = 0x00000200,
    DEFBUTTON4 = 0x00000300,
    SYSTEMMODAL = 0x00001000,
    TASKMODAL = 0x00002000,
    DefaultDesktopOnly = 0x00020000,
    TextAlignRight = 0x00080000,
    RTLREADING = 0x00100000,
    SETFOREGROUND = 0x00010000,
    TOPMOST = 0x00040000,
    ServiceNotification = 0x00200000
}

#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum MessageBoxReturnCode {
    ABORT = 3,
    CANCEL = 2,
    CONTINUE = 11,
    IGNORE = 5,
    NO = 7,
    OK = 1,
    RETRY = 4,
    TryAgain = 10,
    YES = 6,
    UNKNOWN = 0
}

pub fn message_box(title: Option<&str>, description: Option<&str>, box_type: Option<MessageBoxType>, icon_type: Option<MessageBoxIconType>, flags: Option<Vec<MessageBoxFlags>>) -> MessageBoxReturnCode {

    // convert title to LPCSTR
    let mut title_u16: Vec<u16> = title.unwrap_or("")
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    // convert description to LPCSTR
    let mut description_u16: Vec<u16> = description.unwrap_or("")
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    
    // apply message box type
    let mut utype = box_type.unwrap_or(MessageBoxType::OK) as u32;
    if !icon_type.is_none() {
        utype = utype | icon_type.unwrap() as u32;
    }
    
    // apply flags
    if let Some(flag_vec) = flags {
        for flag in flag_vec {
            utype |= flag as u32;
        }
    }
    
    let ret_value: i32;
    unsafe {
        ret_value = user32::MessageBoxW(
            std::ptr::null_mut() as HWND,
            description_u16.as_mut_ptr(),
            title_u16.as_mut_ptr(),
            utype,
        );
    };
    
    match ret_value {
        x if x == MessageBoxReturnCode::ABORT as i32 => MessageBoxReturnCode::ABORT,
        x if x == MessageBoxReturnCode::CANCEL as i32 => MessageBoxReturnCode::CANCEL,
        x if x == MessageBoxReturnCode::CONTINUE as i32 => MessageBoxReturnCode::CONTINUE,
        x if x == MessageBoxReturnCode::IGNORE as i32 => MessageBoxReturnCode::IGNORE,
        x if x == MessageBoxReturnCode::NO as i32 => MessageBoxReturnCode::NO,
        x if x == MessageBoxReturnCode::OK as i32 => MessageBoxReturnCode::OK,
        x if x == MessageBoxReturnCode::RETRY as i32 => MessageBoxReturnCode::RETRY,
        x if x == MessageBoxReturnCode::TryAgain as i32 => MessageBoxReturnCode::TryAgain,
        x if x == MessageBoxReturnCode::YES as i32 => MessageBoxReturnCode::YES,
        _ => MessageBoxReturnCode::UNKNOWN,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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
