use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/google-calendar/events",
        web::get().to(crate::services::google_calendar_service::handle_google_calendar_events),
    );
}
