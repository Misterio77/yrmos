use axum::{
    extract::{Path, Query, State},
    response::Redirect,
    routing::{get, post},
    Form, Router,
};
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use chrono_humanize::{Humanize, Language};
use maud::{html, Markup};
use rust_decimal::Decimal;
use serde::Deserialize;
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
            a role="button" href="/rides/new" { "Oferecer carona" }
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

                    form method="post" action=(format!("/rides/{}/delete", ride.id)) {
                        @let am_driver = if let Some(s) = &session { ride.driver == s.creator } else { false };
                        @if am_driver {
                            button .negative .outline { "Apagar carona" }
                        }
                    }
                }
                section {
                    header { h2 { "Motorista" } }

                    @let profile_link = format!("/profiles/{}", driver.email);
                    a href=(profile_link) {
                        (ACCOUNT_CIRCLE) (format!(" {} ({})", driver.real_name, driver.email))
                    }
                    " "
                    @let positive_rep = driver_rep >= 0;
                    span .highlight .{
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
                    @let is_past = Utc::now() > ride.departure;
                    @let is_full =  riders.len() >= ride.seats.try_into().unwrap_or_default();
                    @let (is_unavailable, unavailable_msg) = match (is_past, is_full) {
                        (true, _) => (true, "Essa corrida já foi finalizada"),
                        (_, true) => (true, "Não há vagas"),
                        _ => (false, ""),
                    };
                    @let already_reserved = if let Some(s) = &session {
                        riders.iter().any(|r| r.email == s.creator)
                    } else { false };

                    @if already_reserved {
                        form method="post" action=(format!("/rides/{}/unreserve", ride.id)) {
                            button .outline { "Desreservar" }
                        }
                    } @else if !am_driver {
                        form method="post" action=(format!("/rides/{}/reserve", ride.id)) {
                            button .outline disabled[is_unavailable] { "Reservar" }
                            (unavailable_msg)
                        }
                    }

                    ul {
                        @for rider in riders {
                            @let profile_link = format!("/profiles/{}", rider.email);
                            li { a href=(profile_link) { (ACCOUNT_CIRCLE) (format!(" {} ({})", rider.real_name, rider.email)) } }
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
        return Err(AppError::NotAllowed);
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
        return Err(AppError::NotAllowed);
    }
    ride.delete_rider(&state.db_pool, &user)
        .await
        .map(|_| Redirect::to(&format!("/rides/{id}")))
}

#[derive(Deserialize)]
struct NewRideForm {
    start_location: String,
    end_location: String,
    #[serde(with = "naive_date")]
    departure_date: NaiveDate,
    #[serde(with = "naive_time")]
    departure_time: NaiveTime,
    seats: i32,
    cost: Option<Decimal>,
    #[serde(default)]
    public: bool,
}

#[derive(Deserialize)]
struct NewRideScreenQuery {
    error: Option<String>,
}

async fn new_ride_screen(session: Session, query: Query<NewRideScreenQuery>) -> Markup {
    let now = Local::now();
    let today = now.format("%Y-%m-%d").to_string();
    let timezone = now.format("GMT%:::z").to_string();

    let main = html! {
        article {
            header {
                h1 { "Nova carona" }
            }
            @if let Some(flash_message) = &query.error { (layouts::flash(flash_message, "error")) }
            form method="post" {
                label {
                    "Local de partida "
                    input name="start_location" required autofocus;
                }
                label {
                    "Local de chegada "
                    input name="end_location" required;
                }
                label {
                    "Dia de saída "
                    input name="departure_date" type="date" value=(today) required;
                }
                label {
                    "Horário de saída "
                    "("
                    (timezone)
                    ") "
                    input name="departure_time" type="time" value=(now.to_string()) required;
                }
                label {
                    "Máximo de passageiros "
                    input name="seats" type="number" value="4" min="1" required;
                }
                label {
                    "Contribuição por passageiro (opcional) "
                    input name="cost" type="number" min="0.01" step="0.01";
                }
                label {
                    input role="switch" type="checkbox" name="public" value="true" checked;
                    "Exibir em buscas?"
                }
                button { "Criar" }
            }
        }
    };
    layouts::default(main, Some(&session))
}

async fn new_ride_action(
    session: Session,
    state: State<AppState>,
    Form(form): Form<NewRideForm>,
) -> Result<Redirect, AppError> {
    let driver = Person::get(&state.db_pool, &session.creator).await?;
    let departure = TimeZone::from_local_datetime(
        &Local,
        &NaiveDateTime::new(form.departure_date, form.departure_time),
    )
    .single()
    .unwrap()
    .into();

    let new_ride = Ride::create(
        &state.db_pool,
        &driver,
        form.seats,
        departure,
        form.start_location,
        form.end_location,
        form.cost,
        form.public,
    )
    .await?;

    Ok(Redirect::to(&format!("/rides/{}", new_ride.id)))
}

pub fn router(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/rides", get(rides_screen))
        .route("/rides/:id", get(ride_screen_by_id))
        .route("/rides/:id/reserve", post(ride_reserve_action))
        .route("/rides/:id/unreserve", post(ride_unreserve_action))
        .route("/rides/new", get(new_ride_screen).post(new_ride_action))
        .with_state(state.clone())
}

mod naive_date {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

mod naive_time {
    use chrono::NaiveTime;
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &'static str = "%H:%M";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
