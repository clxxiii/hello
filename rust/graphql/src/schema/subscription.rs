use std::time::Duration;

use super::Context;
use futures::{StreamExt, stream::BoxStream};
use juniper::FieldResult;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;

pub struct Subscriptions;

type NumberStream = BoxStream<'static, FieldResult<i32>>;

#[juniper::graphql_subscription(Context = Context)]
impl Subscriptions {
    /// Counts seconds.
    async fn count() -> NumberStream {
        let mut value = 0;
        let stream = IntervalStream::new(interval(Duration::from_secs(1))).map(move |_| {
            value += 1;
            Ok(value)
        });
        Box::pin(stream)
    }
}
