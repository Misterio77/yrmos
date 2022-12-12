use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use maud::{html, Markup};
use uuid::Uuid;

use crate::{
    icons::{ACCOUNT_CIRCLE, DIRECTIONS_CAR, SPORTS_SCORE},
    layouts,
    schema::{Ride, Session},
    state::AppState,
    AppError,
};

async fn rides_screen(
    session: Option<Session>,
    state: State<AppState>,
) -> Result<Markup, AppError> {
    let rides = Ride::list_future(&state.db_pool).await?;
    let main = html! {
        @for ride in rides.iter() {
            article {
                header .class {
                    div {
                        h3 { (DIRECTIONS_CAR) }
                        h3 { (SPORTS_SCORE) }
                    }
                    h2 { (ride.departure.to_string()) }
                }
                a href={"/profiles/"(ride.driver)} {
                    span {
                        (ACCOUNT_CIRCLE)
                        " "
                        (ride.driver)
                    }
                }
                footer {
                    a href={"/rides/"(ride.id.to_string())} {
                        "Ver carona →"
                    }
                }
            }
        }
    };
    Ok(layouts::default(main, session.as_ref()))
}

/*
async fn ride_screen_by_id(
    session: Option<Session>,
    Path(id): Path<Uuid>,
    state: &AppState,
) -> Result<Markup, AppError> {
    Ok(html!{})
}
*/

pub fn router(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/rides", get(rides_screen))
        // .route("/rides/:id", get(ride_screen_by_id))
        .with_state(state.clone())
}
