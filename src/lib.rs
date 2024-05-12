use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::MessageBoxW;
use windows::Win32::UI::WindowsAndMessaging::MESSAGEBOX_STYLE;

mod enums;
pub use enums::*;

pub struct Options {
    pub title: String,
    pub description: String,
    pub kind: MessageBoxKind,
    pub icon: Option<MessageBoxIconType>,
    pub flags: Option<Vec<MessageBoxFlags>>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            title: "Message".into(),
            description: "".into(),
            kind: MessageBoxKind::OK,
            icon: None,
            flags: None,
        }
    }
}

pub fn message_box(options: Options) -> Action {
    // convert title to LPCSTR
    let mut title_u16: Vec<u16> = options
        .title
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    // convert description to LPCSTR
    let mut description_u16: Vec<u16> = options
        .description
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    // apply message box type
    let mut utype = options.kind as u32;
    if let Some(icon) = options.icon {
        utype |= icon as u32;
    }

    // apply flags
    if let Some(flag_vec) = options.flags {
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
        )
        .0;
    };

    match ret_value {
        x if x == Action::Abort as i32 => Action::Abort,
        x if x == Action::Cancel as i32 => Action::Cancel,
        x if x == Action::Continue as i32 => Action::Continue,
        x if x == Action::Ignore as i32 => Action::Ignore,
        x if x == Action::NO as i32 => Action::NO,
        x if x == Action::OK as i32 => Action::OK,
        x if x == Action::Retry as i32 => Action::Retry,
        x if x == Action::TryAgain as i32 => Action::TryAgain,
        x if x == Action::Yes as i32 => Action::Yes,
        _ => Action::Uknown,
    }
}

#[cfg(test)]
mod tests;
