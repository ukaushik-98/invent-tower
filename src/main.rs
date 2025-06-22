use std::{pin::Pin, rc::Rc, time::Duration};

use tokio::{net::TcpListener, time::sleep};

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
struct Timeout<T> {
    inner: T,
    duration: Duration,
}

impl<T> Timeout<T> {
    fn new() -> Self {
        todo!()
    }
}

impl<T> Handler for Timeout<T>
where
    T: Handler + Send + Clone,
{
    type Future<'a>
        = Pin<Box<dyn Future<Output = Result<HttpResponse, Error>> + Send + 'a>>
    where
        T: 'a;

    fn call(&mut self, request: HttpRequest) -> Self::Future<'_> {
        // let mut this = self.clone();

        // Box::pin(async move {
        //     let result = tokio::time::timeout(this.duration, this.inner.call(request)).await;

        //     match result {
        //         Ok(Ok(response)) => Ok(response),
        //         Ok(Err(error)) => Err(error),
        //         Err(_timeout) => Err(Error),
        //     }
        // })

        todo!()
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
        // let rc = send_rc().await;
        sleep(Duration::from_millis(10)).await;
    })
    .await;
}
