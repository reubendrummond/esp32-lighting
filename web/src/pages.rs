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

            body {
                main role="main" {
                    div class="container py-4" {
                        (header())
                        (body)
                    }
                }
            }
        }
    }
}

fn header() -> Markup {
    html! {
        header class="pb-3 mb-4 border-bottom" {
            a class="d-flex align-items-center text-dark text-decoration-none" {
                svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="me-2" height="32" {
                    path stroke-linecap="round" stroke-linejoin="round" d="m3.75 13.5 10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75Z" ;
                }
                span class="fs-4" { "ESP32 Lighting" }
            }
        }
    }
}

pub fn index(props: IndexProps) -> Markup {
    page_template(html! {
        section class="jumbotron text-center" {
            div class="container" {
                h1 class="display-5 fw-bold" { "ESP32 Lighting" }

                p class="lead text-muted" { "Light is " (if props.light { "ON" } else { "OFF" }) }

                ul class="nav nav-pills justify-content-center" {
                    li class="mr-1" {
                        a class=(conditional_class("nav-link", "active", props.light)) href="?light=on" { "ON" }
                    }
                    li class="ml-1" {
                        a class=(conditional_class("nav-link", "active", !props.light)) href="?light=off" { "OFF" }
                    }
                }

            }
        }
    })
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
