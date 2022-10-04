use async_stream::stream;
use futures::pin_mut;
use futures::StreamExt;

#[async_trait::async_trait]
trait AsyncTrait {
    async fn yolo(&self) {}
}

struct Stub;

#[async_trait::async_trait]
impl AsyncTrait for Stub {
    async fn yolo(&self) {}
}

#[tokio::main]
async fn main() {
    let s = stream! {
        for i in 0..3 {
            yield i;
        }
    };

    pin_mut!(s); // needed for iteration

    while let Some(value) = s.next().await {
        println!("got {}", value);
    }
}
