use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().route("/", web::get().to(get_index)));

    println!("Serviing on http://localhost:3000");

    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .await
}

// fn get_index() -> HttpResponse {
async fn get_index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="post">
                <input type="text" name="n" />
                <input type="text" name="m" />
                <button type="submit">Compute GCD</button>
            </form>"#,
    )
}
