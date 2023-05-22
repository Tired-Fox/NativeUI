pub enum Buttons {
    OkCancel,
    AbortRetryIgnore,
    CancelTryContinue,
    Help,
    Ok,
    RetryCancel,
    YesNo,
    YesNoCancel,
}

impl Buttons {
    pub fn to_name(&self) -> &str {
        match self {
            Buttons::Ok => "ok",
            Buttons::Help => "help",
            Buttons::YesNo => "yes_no",
            Buttons::OkCancel => "ok_cancel",
            Buttons::RetryCancel => "retry_cancel",
            Buttons::YesNoCancel => "yes_no_cancel",
            Buttons::CancelTryContinue => "cancel_try_continue",
            Buttons::AbortRetryIgnore => "abort_retry_ignore",
        }
    }
}

pub enum Icon {
    Info,
    Warning,
    Question,
    Error,
    None,
}

impl Icon {
    pub fn to_name(&self) -> &str {
        match self {
            Icon::Info => "info",
            Icon::Warning => "warning",
            Icon::Question => "question",
            Icon::Error => "error",
            Icon::None => ""
        }
    }
}

#[derive(PartialEq)]
pub struct MessageReturn(u8);

pub const MESSAGE_SUCCESS: MessageReturn = MessageReturn(1); 
pub const MESSAGE_FAIL: MessageReturn = MessageReturn(0); 
pub const MESSAGE_DISMISS: MessageReturn = MessageReturn(2); 

impl From<MessageReturn> for bool {
    fn from(value: MessageReturn) -> Self {
        if value == MESSAGE_SUCCESS {
            return true
        }
        false
    }
}

mod message;
pub use message::message;
