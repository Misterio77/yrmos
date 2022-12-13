use maud::PreEscaped;

macro_rules! include_icon {
    ($name:ident, $file:expr) => {
        pub const $name: PreEscaped<&'static str> = PreEscaped(include_str!(concat!("../../assets/icons/", $file)));
    }
}

include_icon!(ACCOUNT_CIRCLE, "account_circle.svg");
include_icon!(ARROW_DOWN, "arrow_down.svg");
include_icon!(DIRECTIONS_CAR, "directions_car.svg");
include_icon!(FILTER_LIST, "filter_list.svg");
include_icon!(GRADE, "grade.svg");
include_icon!(LOGIN, "login.svg");
include_icon!(LOGOUT, "logout.svg");
include_icon!(MENU, "menu.svg");
include_icon!(RECOMMEND, "recommend.svg");
include_icon!(SPORTS_SCORE, "sports_score.svg");
include_icon!(THUMB_DOWN, "thumb_down.svg");
include_icon!(THUMBS_UP_DOWN, "thumbs_up_down.svg");
include_icon!(THUMB_UP, "thumb_up.svg");

include_icon!(YRMOS_LOGO, "logo.svg");
