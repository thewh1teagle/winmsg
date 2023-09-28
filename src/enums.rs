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