use maud::Markup;
use rocket::{get, routes};

#[get("/?<light>")]
async fn index(light: Option<String>) -> Markup {
    web::pages::index(web::pages::IndexProps {
        light: match light {
            Some(light) => light == "on",
            None => false,
        },
    })
}

#[rocket::launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![index])
}
