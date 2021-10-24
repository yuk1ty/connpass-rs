use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConnpassResponse {
    results_returned: u32,
    results_available: u32,
    results_start: u32,
    events: Vec<Event>,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    event_id: u32,
    title: String,
    catch: String,
    description: String,
    event_url: String,
    hash_tag: String,
    started_at: String,
    ended_at: String,
    limit: u32,
    event_type: EventType,
    series: Series,
    address: String,
    place: String,
    lat: Option<f32>,
    lon: Option<f32>,
    owner_id: u32,
    owner_nickname: String,
    owner_display_name: String,
    accepted: u32,
    waiting: u32,
    updated_at: String,
}

#[derive(Deserialize, Debug)]
pub enum EventType {
    #[serde(rename = "participation")]
    Participation,
    #[serde(rename = "advertisement")]
    Advertisement,
}

#[derive(Deserialize, Debug)]
pub struct Series {
    id: u32,
    title: String,
    url: String,
}
