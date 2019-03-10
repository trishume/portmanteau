extern crate actix;
extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate tera;

use std::collections::HashMap;
use portmanteau::ListData;

use actix_web::{
    error, http, middleware, server, App, Error, HttpResponse, Query, State,
};

struct AppState {
    template: tera::Tera, // <- store tera template in application state
    small_list: ListData,
    large_list: ListData,
}

fn index(
    (state, query): (State<AppState>, Query<HashMap<String, String>>),
) -> Result<HttpResponse, Error> {
    let s = if let (Some(start), Some(end)) = (query.get("start"), query.get("end")) {
        let mut ctx = tera::Context::new();
        ctx.insert("start", &start.to_owned());
        ctx.insert("end", &end.to_owned());

        let res = state.small_list.searcher().search(start, end).or_else(|_| {
            state.large_list.searcher().search(start, end)
        });
        let text = match res {
            Ok(path) => path.fancy_ouput(),
            Err(_) => "Path not found!  :(".to_owned(),
        };
        ctx.insert("text", &text);
        state
            .template
            .render("result.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
    } else {
        state
            .template
            .render("index.html", &tera::Context::new())
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("tera-example");

    server::new(|| {
        let state = AppState{
            template: compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")),
            small_list: ListData::load_and_index("data/google-10000-english-usa-no-swears.txt").unwrap(),
            large_list: ListData::load_and_index("data/wordlist.asc.txt").unwrap(),
        };
        App::with_state(state)
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.method(http::Method::GET).with(index))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: http://127.0.0.1:8080");
    let _ = sys.run();
}
