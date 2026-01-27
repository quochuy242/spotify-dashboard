use futures::StreamExt;

pub async fn collect_stream<T, U, E, S, F>(mut stream: S, mut map_fn: F) -> Result<Vec<U>, E>
where
    S: futures::Stream<Item = Result<T, E>> + Unpin,
    F: FnMut(T) -> U,
{
    let mut items = Vec::new();

    while let Some(item) = stream.next().await {
        match item {
            Ok(value) => items.push(map_fn(value)),
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(items)
}
