use std::{pin::Pin, time::Duration};

use tokio::time::sleep;

struct HttpRequest;

struct HttpResponse;

struct Error;

trait Handler {
    type Future<'a>: Future<Output = Result<HttpResponse, Error>> + Send + 'a
    where
        Self: 'a;

    fn call(&mut self, request: HttpRequest) -> Self::Future<'_>;
}

#[derive(Clone)]
struct RequestHandler;

impl Handler for RequestHandler {
    type Future<'a> = Pin<Box<dyn Future<Output = Result<HttpResponse, Error>> + Send + 'a>>;

    fn call(&mut self, request: HttpRequest) -> Self::Future<'_> {
        Box::pin(async move { Ok(HttpResponse) })
    }
}

#[derive(Clone)]
struct Timeout {
    inner: RequestHandler,
    duration: Duration,
}

impl Handler for Timeout {
    type Future<'a> = Pin<Box<dyn Future<Output = Result<HttpResponse, Error>> + Send + 'a>>;

    fn call(&mut self, request: HttpRequest) -> Self::Future<'_> {
        Box::pin(async { self.inner.call(request).await })
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let mut t = Timeout {
        inner: RequestHandler,
        duration: Duration::from_millis(100),
    };

    let _ = tokio::spawn(async move {
        let x = t.call(HttpRequest).await;
        sleep(Duration::from_millis(10)).await;
    })
    .await;
}
