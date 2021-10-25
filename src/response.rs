//! Represents the response from connpass API.
//! For more details in https://connpass.com/about/api/.
//! The data class is along with the specification.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ConnpassResponse {
    results_returned: u32,
    results_available: u32,
    results_start: u32,
    events: Vec<Event>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Event {
    event_id: u32,
    title: Option<String>,
    catch: Option<String>,
    description: Option<String>,
    event_url: Option<String>,
    hash_tag: Option<String>,
    started_at: Option<String>,
    ended_at: Option<String>,
    limit: Option<u32>,
    event_type: Option<EventType>,
    series: Option<Series>,
    address: Option<String>,
    place: Option<String>,
    lat: Option<String>,
    lon: Option<String>,
    owner_id: Option<u32>,
    owner_nickname: Option<String>,
    owner_display_name: Option<String>,
    accepted: Option<u32>,
    waiting: Option<u32>,
    updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum EventType {
    #[serde(rename = "participation")]
    Participation,
    #[serde(rename = "advertisement")]
    Advertisement,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Series {
    id: u32,
    title: Option<String>,
    url: Option<String>,
}
