use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sponsor {
    pub hash: String,
    #[serde(rename = "videoID")]
    pub video_id: String,
    pub segments: Vec<Segment>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Segment {
    #[serde(rename = "UUID")]
    pub uuid: String,
    #[serde(rename = "actionType")]
    pub action_type: String,
    pub category: String,
    pub description: String,
    pub locked: i32,
    pub segment: Vec<f32>,
    #[serde(rename = "userID")]
    pub user_id: String,
    #[serde(rename = "videoDuration")]
    pub video_duration: f32,
    pub votes: i32,
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}
