use std::{sync::Arc, time::Duration};

use crate::schema::post::Post;

use super::Context;
use futures::{StreamExt, stream::BoxStream};
use juniper::FieldResult;
use tokio::{sync::broadcast::Receiver, time::interval};
use tokio_stream::wrappers::IntervalStream;

pub struct Subscriptions;

type SubscriptionStream<T> = BoxStream<'static, FieldResult<T>>;

#[juniper::graphql_subscription(Context = Context)]
impl Subscriptions {
    /// Counts seconds.
    async fn count() -> SubscriptionStream<i32> {
        let mut value = 0;
        let stream = IntervalStream::new(interval(Duration::from_secs(1))).map(move |_| {
            value += 1;
            Ok(value)
        });
        Box::pin(stream)
    }

    /// Streams the number of likes on a post
    async fn post_likes(context: &Context) -> SubscriptionStream<Arc<Post>> {
        let mut rx = context.post_likes_broadcast.subscribe();
        receiver_to_stream(rx)
    }
}

fn receiver_to_stream<T: Clone + Send + 'static>(mut rx: Receiver<T>) -> SubscriptionStream<T> {
    let stream = futures::stream::unfold(rx, |mut rx| async move {
        match rx.recv().await {
            Ok(x) => Some((Ok(x), rx)),
            Err(_) => None,
        }
    });
    Box::pin(stream)
}
