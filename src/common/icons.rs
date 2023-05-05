use super::files::include_asset_file;
use maud::PreEscaped;

macro_rules! mk_icon {
    ($name:ident, $file:expr) => {
        pub const $name: PreEscaped<&'static str> = PreEscaped(include_asset_file!($file));
    };
}

mk_icon!(ACCOUNT_CIRCLE, "account_circle.svg");
mk_icon!(ARROW_DOWN, "arrow_down.svg");
mk_icon!(DIRECTIONS_CAR, "directions_car.svg");
mk_icon!(FILTER_LIST, "filter_list.svg");
mk_icon!(GRADE, "grade.svg");
mk_icon!(LOGIN, "login.svg");
mk_icon!(LOGOUT, "logout.svg");
mk_icon!(MENU, "menu.svg");
mk_icon!(RECOMMEND, "recommend.svg");
mk_icon!(SPORTS_SCORE, "sports_score.svg");
mk_icon!(THUMB_DOWN, "thumb_down.svg");
mk_icon!(THUMBS_UP_DOWN, "thumbs_up_down.svg");
mk_icon!(THUMB_UP, "thumb_up.svg");

mk_icon!(YRMOS_LOGO, "logo.svg");
