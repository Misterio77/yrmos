use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use chrono::{DateTime, Local, Utc};
use chrono_humanize::{Humanize, Language};
use maud::{html, Markup, PreEscaped};
use tokio::try_join;
use uuid::Uuid;

use crate::{
    icons::{ACCOUNT_CIRCLE, ARROW_DOWN, DIRECTIONS_CAR, SPORTS_SCORE},
    layouts,
    schema::{Person, Ride, Session},
    state::AppState,
    AppError,
};

async fn rides_screen(
    session: Option<Session>,
    state: State<AppState>,
) -> Result<Markup, AppError> {
    let rides = Ride::list_future(&state.db_pool).await?;
    let main = html! {
        section {
            header { h1 { "Caronas" }}
            .grid {
                @for ride in rides.iter() {
                    @let departure_local: DateTime<Local> = ride.departure.into();
                    @let departure_pretty = departure_local.format("%H:%M %d/%m (GMT%:::z)").to_string();
                    @let departure_humanized = (ride.departure - Utc::now()).humanize_in(Language::Portuguese);
                    @let profile_link = format!("/profiles/{}", ride.driver);
                    @let ride_link = format!("/rides/{}", ride.id);
                    article {
                        header .row {
                            h3 {
                                (DIRECTIONS_CAR) " " (ride.start_location)
                                br;
                                span .muted { (ARROW_DOWN) }
                                br;
                                (SPORTS_SCORE) " " (ride.end_location)
                            }
                            h2 { span data-tooltip=(departure_pretty) { (departure_humanized) } }
                        }
                        a href=(profile_link) { (ACCOUNT_CIRCLE) " " (ride.driver) }
                        footer { a role="button" href=(ride_link) { "Ver carona →" } }
                    }
                }
            }
        }
    };
    Ok(layouts::default(main, session.as_ref()))
}

async fn ride_screen_by_id(
    session: Option<Session>,
    Path(id): Path<Uuid>,
    state: State<AppState>,
) -> Result<Markup, AppError> {
    let ride = Ride::get(&state.db_pool, id).await?;
    let (driver, riders) = try_join!(
        ride.get_driver(&state.db_pool),
        ride.get_riders(&state.db_pool)
    )?;
    let departure_local: DateTime<Local> = ride.departure.into();
    let departure_pretty = departure_local.format("%H:%M %d/%m (GMT%:::z)").to_string();
    let departure_humanized = (ride.departure - Utc::now()).humanize_in(Language::Portuguese);

    let short_uuid = format!("{:x}", ride.id.as_fields().0);
    let qr_pix = driver.get_pix_qr(ride.cost);

    let main = html! {
        article {
            header {
                h1 {
                    "Carona: " code { (short_uuid) }
                }
                div .row {
                    h3 {
                        (DIRECTIONS_CAR) " " (ride.start_location)
                        br;
                        span .muted { (ARROW_DOWN) }
                        br;
                        (SPORTS_SCORE) " " (ride.end_location)
                    }
                    h2 { span data-tooltip=(departure_pretty) { (departure_humanized) } }
                }
                @if let Some(cost) = ride.cost {
                    p { "R$ " (cost.to_string()) }
                }
                @let profile_link = format!("/profiles/{}", driver.email);
                p { a href=(profile_link) { (ACCOUNT_CIRCLE) (format!(" {} ({})", driver.real_name, driver.email)) } }
            }
            h2 { "Passageiros" }
            ul {
                @for rider in riders {
                    @let profile_link = format!("/profiles/{}", rider.email);
                    li { a href=(profile_link) { (ACCOUNT_CIRCLE) (format!(" {} ({})", rider.real_name, rider.email)) }}
                }
            }
            footer {
                h2 { "Pagamento (via Pix)" }
                @if let Some(qr) = qr_pix {
                    img width="256" height="256" src={
                        "https://qrcode.tec-it.com/API/QRCode?data="
                        (qr)
                        "&color=222&backcolor=eee&quietzone=2"
                    };
                    hr;
                    pre { code { (qr) } }
                } @else {
                    p { "O motorista ainda não cadastrou uma chave pix." }
                }
            }

        }
    };
    // img src=(format!({"https://"}))
    Ok(layouts::default(main, session.as_ref()))
}

pub fn router(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/rides", get(rides_screen))
        .route("/rides/:id", get(ride_screen_by_id))
        .with_state(state.clone())
}
