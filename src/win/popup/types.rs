use windows::Win32::UI::WindowsAndMessaging::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Icon {
    Info,
    Warning,
    Question,
    Error,
    None,
}

impl From<Icon> for MESSAGEBOX_STYLE {
    fn from(value: Icon) -> Self {
        match value {
            Icon::Info => MB_ICONINFORMATION,
            Icon::Question => MB_ICONQUESTION,
            Icon::Warning => MB_ICONWARNING,
            Icon::Error => MB_ICONERROR,
            _ => return MESSAGEBOX_STYLE::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ButtonLayout {
    OkCancel,
    AbortRetryIgnore,
    CancelTryContinue,
    Help,
    Ok,
    RetryCancel,
    YesNo,
    YesNoCancel,
}

impl From<ButtonLayout> for MESSAGEBOX_STYLE {
    fn from(value: ButtonLayout) -> Self {
        match value {
            ButtonLayout::Ok => MB_OK,
            ButtonLayout::Help => MB_HELP,
            ButtonLayout::YesNo => MB_YESNO,
            ButtonLayout::OkCancel => MB_OKCANCEL,
            ButtonLayout::RetryCancel => MB_RETRYCANCEL,
            ButtonLayout::YesNoCancel => MB_YESNOCANCEL,
            ButtonLayout::CancelTryContinue => MB_CANCELTRYCONTINUE,
            ButtonLayout::AbortRetryIgnore => MB_ABORTRETRYIGNORE,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MessageReturn {
    Ok,
    Yes,
    Continue,
    No,
    Cancel,
    Abort,
    Ignore,
    Retry,
    TryAgain,
    Help,
    Close,
    Async,
    Timeout,
}

impl From<MESSAGEBOX_RESULT> for MessageReturn {
    fn from(value: MESSAGEBOX_RESULT) -> Self {
        match value {
            IDOK => MessageReturn::Ok,
            IDYES => MessageReturn::Yes,
            IDCONTINUE => MessageReturn::Continue,
            IDNO => MessageReturn::No,
            IDCANCEL => MessageReturn::Cancel,
            IDABORT => MessageReturn::Abort,
            IDIGNORE => MessageReturn::Ignore,
            IDRETRY => MessageReturn::Retry,
            IDTRYAGAIN => MessageReturn::TryAgain,
            IDHELP => MessageReturn::Help,
            IDCLOSE => MessageReturn::Close,
            IDASYNC => MessageReturn::Async,
            IDTIMEOUT => MessageReturn::Timeout,
            _ => MessageReturn::Abort
        }
    }
}


impl From<MessageReturn> for MESSAGEBOX_RESULT {
    fn from(value: MessageReturn) -> Self {
        match value {
            MessageReturn::Ok => IDOK,
            MessageReturn::Yes => IDYES,
            MessageReturn::Continue => IDCONTINUE,
            MessageReturn::No => IDNO,
            MessageReturn::Cancel => IDCANCEL,
            MessageReturn::Abort => IDABORT,
            MessageReturn::Ignore => IDIGNORE,
            MessageReturn::Retry => IDRETRY,
            MessageReturn::TryAgain => IDTRYAGAIN,
            MessageReturn::Help => IDHELP,
            MessageReturn::Close => IDCLOSE,
            MessageReturn::Async => IDASYNC,
            MessageReturn::Timeout => IDTIMEOUT,
        }
    }
}

