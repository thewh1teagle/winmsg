pub enum MessageBoxIconType {
    Warning = 0x00000030,
    Information = 0x00000040,
    Question = 0x00000020,
    Error = 0x00000010,
}

pub enum MessageBoxKind {
    AbortRetryIgnore = 0x00000002,
    CancelTryContinue = 0x00000006,
    Help = 0x00004000,
    OK = 0x00000000,
    OkCancel = 0x00000001,
    RetryCancel = 0x00000005,
    YesNo = 0x00000004,
    YesNoCancel = 0x00000003,
}

pub enum MessageBoxFlags {
    DefButton1 = 0x00000000,
    DefButton2 = 0x00000100,
    DefButton3 = 0x00000200,
    DefButton4 = 0x00000300,
    SystemModal = 0x00001000,
    TaskModal = 0x00002000,
    DefaultDesktopOnly = 0x00020000,
    TextAlignRight = 0x00080000,
    Rtlreading = 0x00100000,
    Setforeground = 0x00010000,
    Topmost = 0x00040000,
    ServiceNotification = 0x00200000,
}

#[derive(Copy, Clone, Debug)]
pub enum Action {
    Abort = 3,
    Cancel = 2,
    Continue = 11,
    Ignore = 5,
    NO = 7,
    OK = 1,
    Retry = 4,
    TryAgain = 10,
    Yes = 6,
    Uknown = 0,
}
