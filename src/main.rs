mod mocks;
mod parser;
mod queries;
mod schema;
mod utils;

use std::{io, sync::Arc};

use actix_cors::Cors;
use actix_web::{
    get, middleware, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use mocks::{get_current_report, get_historical_report};
use parser::parse_reports;
use schema::Context;

use crate::schema::{create_schema, Schema};

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(schema: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    match parse_reports(vec![get_current_report(), get_historical_report()]) {
        Some(reports) => {
            let context = Context { reports };
            let report = data.execute(&schema, &context).await;
            HttpResponse::Ok().json(report)
        }
        None => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let schema = Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
