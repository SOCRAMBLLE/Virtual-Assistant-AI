use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use serde_json::Value;

pub struct GoogleCalendarService {
    client: reqwest::Client,
    access_token: String,
}

impl GoogleCalendarService {
    pub fn new(access_token: String) -> Self {
        GoogleCalendarService {
            client: reqwest::Client::new(),
            access_token,
        }
    }

    /// Lista os eventos do calendário primário.
    pub async fn list_events(&self) -> Result<Value, reqwest::Error> {
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

pub async fn handle_google_calendar_events(
    google_calendar_service: web::Data<GoogleCalendarService>,
    id: Identity,
) -> impl Responder {
    match id.id() {
        Ok(access_token) => {
            // Aqui você pode usar o access_token conforme necessário
            let service = google_calendar_service.get_ref();
            println!("Usuário autenticado: {:?}", id.id());
            println!("Usuário autenticado: {:?}", access_token);
            match service.list_events().await {
                Ok(events) => HttpResponse::Ok().json(events),
                Err(e) => {
                    eprintln!("Error fetching Google Calendar events: {}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}
