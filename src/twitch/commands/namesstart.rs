use super::*;

/// List current chatters in a channel. (marks the start begin)
///
/// If there are more than 1000 chatters in a room, NAMES return only the list
/// of operator privileges currently in the room.
///
/// The server will send these until it sends a NamesEnd for the same channel
///
/// Listen for this and keep track of the users and once you received NamedEnd
/// you've gotten all of the users
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamesStart {
    pub(super) user: String,
    pub(super) channel: Channel,
    pub(super) users: Vec<String>,
}

impl NamesStart {
    /// Your user for this event
    pub fn user(&self) -> &str {
        &self.user
    }
    /// The channel this event is happening on
    pub fn channel(&self) -> &Channel {
        &self.channel
    }
    /// List of users returned by this
    pub fn users(&self) -> &[String] {
        &self.users
    }
}
