use windows::Win32::UI::WindowsAndMessaging::MessageBoxW;
use windows::Win32::Foundation::HWND;
use windows::core::PCWSTR;
use windows::Win32::UI::WindowsAndMessaging::MESSAGEBOX_STYLE;

mod enums;
use enums::*;


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
        ret_value = MessageBoxW(
            HWND(0),
            PCWSTR(description_u16.as_mut_ptr()),
            PCWSTR(title_u16.as_mut_ptr()),
            MESSAGEBOX_STYLE(utype),
        ).0;
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
mod tests;