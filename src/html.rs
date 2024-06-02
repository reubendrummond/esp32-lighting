use maud::{html, Markup};

fn page_template(body: Markup) -> Markup {
    html! {
        html {
            head {
                meta charset="utf-8";
                title { "ESP32 Lighting" }
                link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css";
                link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/bootswatch/5.3.3/lux/bootstrap.min.css";
                script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js" {};
            }
            body class="container" {
                (body)
            }
        }
    }
}

pub struct IndexProps {
    pub light: bool,
}

fn conditional_class(base_class: &str, optional_class: &str, condition: bool) -> String {
    if condition {
        format!("{} {}", base_class, optional_class)
    } else {
        base_class.to_string()
    }
}

pub fn index(props: IndexProps) -> Markup {
    page_template(html! {
        h1 { "Welcome to ESP32 lighting!" }

        h2 { "Light is " (if props.light { "ON" } else { "OFF" }) }

        ul class="nav" {
            li class="nav-item" {
                a class=(conditional_class("nav-link", "active", props.light)) href="/?light=on" { "ON" }
            }
            li class="nav-item" {
                a class=(conditional_class("nav-link", "active", props.light)) href="/?light=off" { "OFF" }
             }
        }

    })
}
