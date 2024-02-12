use reqwest::{Client, Error};
use serde_json::Value;

pub struct GoogleCalendarService {
    client: Client,
    access_token: String,
}

impl GoogleCalendarService {
    pub fn new(access_token: String) -> Self {
        GoogleCalendarService {
            client: Client::new(),
            access_token,
        }
    }

    /// Lista os eventos do calendário primário.
    pub async fn list_events(&self) -> Result<Value, Error> {
        let url = "https://www.googleapis.com/calendar/v3/calendars/primary/events";
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;

        let events = response.json::<Value>().await?;
        Ok(events)
    }
}
