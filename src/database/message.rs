use crate::extra::WQEvent;
use ricq::structs::{
    FriendAudioMessage, FriendMessage, GroupAudioMessage, GroupMessage, GroupTempMessage,
    MessageReceipt,
};
use serde::{Deserialize, Serialize};
pub trait MessageId {
    fn seq(&self) -> i32;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SMessage {
    Group(SGroupMessage),
    Private(SPrivateMessage),
}

impl SMessage {
    pub fn event(self) -> WQEvent {
        match self {
            SMessage::Group(group) => group.event,
            SMessage::Private(private) => private.event,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SGroupMessage {
    pub seqs: Vec<i32>,
    pub rands: Vec<i32>,
    pub group_code: i64,
    pub event: WQEvent,
}

impl MessageId for SGroupMessage {
    fn seq(&self) -> i32 {
        self.seqs[0]
    }
}

impl SGroupMessage {
    pub fn new(m: GroupMessage, event: WQEvent) -> Self {
        Self {
            seqs: m.seqs,
            rands: m.rands,
            group_code: m.group_code,
            event,
        }
    }

    pub fn receipt(receipt: MessageReceipt, group_code: i64, event: WQEvent) -> Self {
        Self {
            seqs: receipt.seqs,
            rands: receipt.rands,
            group_code,
            event,
        }
    }

    pub fn from_audio_event(m: GroupAudioMessage, event: WQEvent) -> Self {
        Self {
            seqs: m.seqs,
            rands: m.rands,
            group_code: m.group_code,
            event,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SPrivateMessage {
    pub seqs: Vec<i32>,
    pub rands: Vec<i32>,
    pub target_id: i64,
    pub time: i64,
    pub event: WQEvent,
}

impl MessageId for SPrivateMessage {
    fn seq(&self) -> i32 {
        self.seqs[0]
    }
}

impl SPrivateMessage {
    pub fn new(m: FriendMessage, event: WQEvent) -> Self {
        Self {
            seqs: m.seqs,
            rands: m.rands,
            target_id: m.target,
            time: m.time as i64,
            event,
        }
    }

    pub fn from_temp(m: GroupTempMessage, event: WQEvent) -> Self {
        Self {
            seqs: m.seqs,
            rands: m.rands,
            target_id: m.from_uin,
            time: m.time as i64,
            event,
        }
    }

    pub fn receipt(receipt: MessageReceipt, target_id: i64, event: WQEvent) -> Self {
        Self {
            seqs: receipt.seqs,
            rands: receipt.rands,
            target_id,
            time: receipt.time,
            event,
        }
    }

    pub fn from_audio_event(m: FriendAudioMessage, event: WQEvent) -> Self {
        Self {
            seqs: m.seqs,
            rands: m.rands,
            target_id: m.target,
            time: m.time as i64,
            event,
        }
    }
}
