//! Update parsing for 'room' update records.
//!
//! This is made significantly more complicated by the fact that all updates besides the initial one are "partial" -
//! they only contain changes, and each update to a specific room object will not contain the object's type, as it
//! will not have changed.
use std::collections::HashMap;
use std::marker::PhantomData;

use data::Badge;

use {serde_json, tuple_vec_map};

pub mod flags;
#[macro_use]
mod room_object_macros;
#[macro_use]
pub mod resources;
pub mod objects;

use self::flags::{deserialize_flags, Flag};

/// Update for detailed room information.
#[derive(serde_derive::Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoomUpdate {
    /// The game time when this update was created.
    pub game_time: Option<u32>,
    /// Information on the room "mode".
    pub info: RoomUpdateInfo,
    /// All room objects in the room, represented as serde_json::Value.
    ///
    /// This would be parsed into a concrete enum for each type, but room
    /// updates by their nature are incremental - and this includes the "type"
    /// field.
    ///
    /// These values can be applied as updates to the `RoomObject` type.
    #[serde(with = "tuple_vec_map")]
    pub objects: Vec<(String, serde_json::Value)>,
    /// All of the subscribed user's flags in this room.
    ///
    /// This will always be present when there are flags, even if
    /// no flags have changed.
    #[serde(deserialize_with = "deserialize_flags")]
    #[serde(default)]
    pub flags: Vec<Flag>,
    /// The logged in user's visuals for this room.
    ///
    /// Represented by a series of json objects separated by `\n`.
    ///
    /// TODO: parse this further.
    pub visual: Option<String>,
    /// Detailed information on all users that have things in this room.
    pub users: Option<HashMap<String, RoomUpdateUserInfo>>,
    /// Phantom data in order to allow adding any additional fields in the future.
    #[serde(skip)]
    _phantom: PhantomData<()>,
}

/// "info" struct to go with room update.
///
/// TODO: find all variants and parse into enum.
#[derive(serde_derive::Deserialize, Clone, Hash, Debug)]
pub struct RoomUpdateInfo {
    /// Usually "world" for regular rooms.
    pub mode: Option<String>,
    /// Phantom data in order to allow adding any additional fields in the future.
    #[serde(skip)]
    _phantom: PhantomData<()>,
}

/// Information on a user which is packaged with a room update.
#[derive(serde_derive::Deserialize, Clone, Hash, Debug)]
pub struct RoomUpdateUserInfo {
    /// User ID
    #[serde(rename = "_id")]
    pub user_id: Option<String>,
    /// Username
    pub username: Option<String>,
    /// Badge description
    pub badge: Option<Badge>,
    /// Phantom data in order to allow adding any additional fields in the future.
    #[serde(skip)]
    _phantom: PhantomData<()>,
}
