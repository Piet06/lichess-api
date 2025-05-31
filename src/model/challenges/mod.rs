pub mod accept;
pub mod add_time;
pub mod ai;
pub mod cancel;
pub mod create;
pub mod decline;
pub mod list;
pub mod open;
pub mod start_clocks;

use crate::model::{Color, Compat, Days, LightUser, Speed, Title, Variant, VariantKey};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Clone, Debug, Serialize)]
pub struct OpenChallenge {
    #[serde(flatten)]
    pub base: ChallengeBase,
    pub name: String,
    pub rules: String,
    pub users: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateChallenge {
    #[serde(flatten)]
    pub base: ChallengeBase,
    pub rated: bool,
    pub keep_alive_stream: bool,
    pub accept_by_token: Option<String>,
    pub message: Option<String>,
    pub rules: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Rules {
    NoAbort,
    NoRematch,
    NoGiveTime,
    NoClaimWin,
}

#[derive(Clone, Debug, Serialize)]
pub struct AIChallenge {
    #[serde(flatten)]
    pub base: ChallengeBase,
    pub level: u32,
    pub color: Color,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct ChallengeBase {
    #[serde(rename = "clock.limit")]
    pub clock_limit: Option<u32>,
    #[serde(rename = "clock.increment")]
    pub clock_increment: Option<u32>,
    pub days: Option<Days>,
    pub variant: VariantKey,
    pub fen: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeOpenJson {
    #[serde(flatten)]
    pub base: ChallengeJsonBase,
    pub url_white: String,
    pub url_black: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeCreated {
    #[serde(flatten)]
    pub challenge: ChallengeJson,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeJson {
    #[serde(flatten)]
    pub base: ChallengeJsonBase,
    pub initial_fen: Option<String>,
    pub decline_reason: Option<String>,
    pub decline_reason_key: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeJsonBase {
    pub id: String,
    pub url: String,
    pub color: Color,
    pub direction: Option<Direction>,
    pub final_color: String,
    pub time_control: TimeControl,
    pub variant: Variant,

    /// The api docs suggest this is non-nullable,
    /// this is true only if the user does not accept anonymous challengers.
    pub challenger: Option<ChallengeUser>,
    pub dest_user: Option<ChallengeUser>,
    pub perf: Perf,
    pub rated: bool,
    pub speed: Speed,
    pub status: Status,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Direction {
    In,
    Out,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Perf {
    pub icon: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Created,
    Offline,
    Canceled,
    Declined,
    Accepted,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum TimeControl {
    Clock {
        increment: u32,
        limit: u32,
        show: String,
    },
    Correspondence {
        days_per_turn: u32,
    },
    Unlimited,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChallengeUser {
    #[serde(flatten)]
    pub user: LightUser,
    pub rating: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisional: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag: Option<u32>,
    pub title: Option<Title>,
}
