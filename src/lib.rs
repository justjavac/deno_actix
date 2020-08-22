extern crate actix_web;
extern crate deno_core;
extern crate futures;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use deno_core::plugin_api::{Buf, Interface, Op, ZeroCopyBuf};
use futures::future::FutureExt;

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
  interface.register_op("run", op_run);
}

fn op_run(
  _interface: &mut dyn Interface,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Op {
  main();
  let result: Buf = None;
  Op::Sync(result)
}

async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

async fn foo() -> impl Responder {
  HttpResponse::Ok().body("Hello world again!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .route("/", web::get().to(index))
      .route("/again", web::get().to(foo))
  })
  .bind("127.0.0.1:8088")?
  .run()
  .await
}
