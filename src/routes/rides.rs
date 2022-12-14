use axum::{
    extract::{Path, State},
    response::Redirect,
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Local, Utc};
use chrono_humanize::{Humanize, Language};
use maud::{html, Markup, PreEscaped};
use tokio::try_join;
use uuid::Uuid;

use crate::{
    icons::{ACCOUNT_CIRCLE, ARROW_DOWN, DIRECTIONS_CAR, SPORTS_SCORE, THUMB_DOWN, THUMB_UP},
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
                    @let ride_link = format!("/rides/{}", ride.id);
                    article {
                        header {
                            .row {
                                h3 {
                                    (DIRECTIONS_CAR) " " (ride.start_location)
                                    br;
                                    span .muted { (ARROW_DOWN) }
                                    br;
                                    (SPORTS_SCORE) " " (ride.end_location)
                                }
                                h2 { span data-tooltip=(departure_pretty) { (departure_humanized) } }
                            }
                            h3 {
                                "Preço: "
                                @if let Some(cost) = ride.cost {
                                    code { "R$" (cost.to_string()) }
                                } @else {
                                    "a combinar"
                                }
                            }
                        }
                        a role="button" href=(ride_link) { "Ver carona →" }
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
    let driver_rep = driver.get_reputation(&state.db_pool).await?;
    let departure_local: DateTime<Local> = ride.departure.into();
    let departure_pretty = departure_local.format("%H:%M %d/%m (GMT%:::z)").to_string();
    let departure_humanized = (ride.departure - Utc::now()).humanize_in(Language::Portuguese);

    let short_uuid = format!("{:x}", ride.id.as_fields().0);
    let qr_pix = driver.get_pix_qr(ride.cost);

    let main = html! {
        section {
            header {
                h1 {
                    "Carona: " code { (short_uuid) }
                }
            }
            article {
                header {
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
                    h3 {
                        "Preço: "
                        @if let Some(cost) = ride.cost {
                            code { "R$" (cost.to_string()) }
                        } @else {
                            "a combinar"
                        }
                    }
                }
                section {
                    header { h2 { "Motorista" } }

                    @let am_driver = if let Some(s) = &session { ride.driver == s.creator } else { false };
                    @if am_driver {
                        form method="post" .negative action=(format!("/rides/{}/delete", ride.id)) {
                            button .small { "Apagar carona" }
                        }
                    }

                    @let profile_link = format!("/profiles/{}", driver.email);
                    a href=(profile_link) {
                        (ACCOUNT_CIRCLE) (format!(" {} ({})", driver.real_name, driver.email))
                    }
                    " "
                    @let positive_rep = driver_rep >= 0;
                    span .{
                        @if positive_rep { "positive" } @else { "negative" }
                    } {
                        @if positive_rep { (THUMB_UP) } @else { (THUMB_DOWN) }
                        " "
                        (driver_rep)
                    }
                }
                section {
                    header {
                        h2 { (format!("Passageiros ({}/{})", riders.len(), ride.seats)) }
                    }
                    @let departed = Utc::now() > ride.departure;
                    @let is_full =  riders.len() >= ride.seats.try_into().unwrap_or_default();
                    @let already_reserved = if let Some(s) = &session {
                        riders.iter().any(|r| r.email == s.creator)
                    } else { false };

                    @if already_reserved {
                        form method="post" .negative action=(format!("/rides/{}/unreserve", ride.id)) {
                                button .small { "Desreservar" }
                        }
                    } @else {
                        form method="post" action=(format!("/rides/{}/reserve", ride.id)) {
                                button .small disabled[(departed || is_full || am_driver)] { "Reservar" }
                        }
                    }

                    ul {
                        @for rider in riders {
                            @let profile_link = format!("/profiles/{}", rider.email);
                            li { a href=(profile_link) { (ACCOUNT_CIRCLE) (format!(" {} ({})", rider.real_name, rider.email)) }}
                        }
                    }
                }
                footer {
                    h2 { "Pagamento (via Pix)" }
                    @if let Some(qr) = qr_pix {
                        img width="256" height="256" loading="lazy" src={
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
        }
    };
    // img src=(format!({"https://"}))
    Ok(layouts::default(main, session.as_ref()))
}

async fn ride_reserve_action(
    session: Session,
    Path(id): Path<Uuid>,
    state: State<AppState>,
) -> Result<Redirect, AppError> {
    let (ride, user) = try_join!(
        Ride::get(&state.db_pool, id),
        Person::get(&state.db_pool, &session.creator)
    )?;
    if ride.driver == session.creator {
        return Err(AppError::NotAllowed)
    }
    ride.insert_rider(&state.db_pool, &user)
        .await
        .map(|_| Redirect::to(&format!("/rides/{id}")))
}

async fn ride_unreserve_action(
    session: Session,
    Path(id): Path<Uuid>,
    state: State<AppState>,
) -> Result<Redirect, AppError> {
    let (ride, user) = try_join!(
        Ride::get(&state.db_pool, id),
        Person::get(&state.db_pool, &session.creator)
    )?;
    if ride.driver == session.creator {
        return Err(AppError::NotAllowed)
    }
    ride.delete_rider(&state.db_pool, &user)
        .await
        .map(|_| Redirect::to(&format!("/rides/{id}")))
}

pub fn router(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/rides", get(rides_screen))
        .route("/rides/:id", get(ride_screen_by_id))
        .route("/rides/:id/reserve", post(ride_reserve_action))
        .route("/rides/:id/unreserve", post(ride_unreserve_action))
        .with_state(state.clone())
}
