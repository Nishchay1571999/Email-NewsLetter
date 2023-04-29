use actix_web::{get,HttpResponse,HttpServer,Responder,App};


#[get("/")]
async fn hello() -> impl Responder{
    HttpResponse::Ok().body("Hello World!")
}

#[get("/echo")]
async fn manual_hello() -> impl Responder{
    HttpResponse::Ok().body("Hello World!!!!")
}




#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||
    {
        App::new()
            .service(hello)
            .service(manual_hello)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}


