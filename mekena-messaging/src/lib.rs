pub mod mailbox;
pub mod message;

pub mod prelude {
    pub use crate::mailbox::{Mailbox, MailboxError};
    pub use crate::message::Message;
}
