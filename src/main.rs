mod middlewares;
mod theme;
mod config;

use actix_files::Files;
use actix_web::cookie::Cookie;
use actix_web::{web, App, HttpResponse, HttpServer, Error, HttpRequest, http};
use askama::Template;
use serde::Deserialize;

#[derive(Deserialize, Template)]
#[template(path = "login.html")]
struct LoginForm {
    username: String,
    password: String,
    error_message: Option<String>,
}

impl LoginForm {
    pub fn new() -> Self {
        LoginForm {
            username: "".to_string(),
            password: "".to_string(),
            error_message: None,
        }
    }
}

#[derive(Template)]
#[template(path = "secured/template/base.html")]
struct BaseTemplate;

#[derive(Template)]
#[template(path = "secured/views/dashboard.html")]
struct DashboardTemplate;

#[derive(Template)]
#[template(path = "secured/views/users/search.html")]
struct UsersTemplate;

#[derive(Template)]
#[template(path = "secured/views/users/new-user.html")]
struct NewUserTemplate;


#[derive(Template)]
#[template(path = "secured/views/profile.html")]
struct ProfileTemplate;

#[derive(Template)]
#[template(path = "secured/views/notifications.html")]
struct NotificationsTemplate;

#[derive(Template)]
#[template(path = "secured/views/theme-settings.html")]
struct ThemeSettingsTemplate;

#[derive(Template)]
#[template(path = "errors/404.html")]
struct NotFoundTemplate;



// middleware
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(middlewares::InternalServerErrorMiddleware)
            .service(
                web::resource("/")
                    .route(web::get().to(login))
                    .route(web::post().to(authenticate)),
            )
            .route("/dashboard", web::get().to(dashboard))
            .route("/logout", web::get().to(logout))
            // users routes
            .service(web::resource("/users").route(web::get().to(users)))
            .service(web::resource("/users/new-user").route(web::get().to(user_new)))
            .service(web::resource("/profile").route(web::get().to(render_profile)))
            .service(web::resource("/notifications").route(web::get().to(render_notifications)))
            .service(web::resource("/theme-settings").route(web::get().to(theme_settings)))
            // static resource
            .service(Files::new("/css", "./static/css"))
            .service(Files::new("/js", "./static/js"))
            // error handlers
            .default_service(  web::to(resource_not_found))


    })
    .bind(("127.0.0.1", 8181))?
    .run()
    .await
}

async fn resource_not_found(_req: HttpRequest) -> Result<HttpResponse, Error> {
    let s = NotFoundTemplate.render().unwrap();
    Ok(HttpResponse::NotFound().content_type("text/html").body(s))
}



async fn login() -> Result<HttpResponse, Error> {
    println!(" showing login page ....");
    let s = LoginForm::new().render().unwrap();
    Ok(HttpResponse::Ok().body(s))
}

async fn logout() -> HttpResponse {
    println!(" logout  ....");
    HttpResponse::SeeOther()
        .insert_header((http::header::LOCATION, "/"))
        .finish()
}

async fn authenticate(form: web::Form<LoginForm>) -> Result<HttpResponse, Error> {
    println!("processing authentication .... username: {}", form.username);
    if form.username == "ram" {
        Ok(HttpResponse::SeeOther()
            .cookie(
                Cookie::build("my_auth_cookie", "SomeValue")
                    .http_only(true) // for security
                    .finish(),
            )
            .insert_header((http::header::LOCATION, "/dashboard"))
            .finish())
    } else {
        let s = LoginForm {
            username: form.username.clone(),
            password: "".to_string(),
            error_message: Some("Invalid credentials".to_string()),
        }
        .render()
        .unwrap();
        Ok(HttpResponse::Ok().body(s))
    }
}

async fn dashboard() -> HttpResponse {
    let s = DashboardTemplate.render().unwrap();
    HttpResponse::Ok().body(s)
}

async fn users() -> Result<HttpResponse, Error> {
    let s = UsersTemplate.render().unwrap();
    Ok(HttpResponse::Ok().body(s))
}

async fn user_new() ->  Result<HttpResponse, Error> {
    let s = NewUserTemplate.render().unwrap();
    Ok(HttpResponse::Ok().body(s))
}

async fn render_profile() ->  Result<HttpResponse, Error> {
    let s = ProfileTemplate.render().unwrap();
    Ok(HttpResponse::Ok().body(s))
}

async fn render_notifications() ->  Result<HttpResponse, Error> {
    let s = NotificationsTemplate.render().unwrap();
    Ok(HttpResponse::Ok().body(s))
}

async fn theme_settings() -> Result<HttpResponse, Error> {
    let s = ThemeSettingsTemplate.render().unwrap();
    Ok(HttpResponse::Ok().body(s))
}