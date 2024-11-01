use crate::hyperlight::{SANDBOX_POOL, MULTI_USE_SANDBOX_POOL, COUNTER};
use hyperlight_common::flatbuffer_wrappers::function_types::ReturnType;
use std::sync::atomic::Ordering;
use warp::Filter;

pub(crate) fn deref_raw_null_ptr() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("hyperlight")
        .and(warp::path("safety"))
        .and(warp::path("deref-raw-null-ptr"))
        .and_then(move || async {
            // Acquire a permit to access the pool
            let _permit = SANDBOX_POOL.acquire().await.unwrap();

            // Select a sandbox based on the round-robin counter
            let index = COUNTER.fetch_add(1, Ordering::SeqCst) % MULTI_USE_SANDBOX_POOL.len();
            let sandbox = MULTI_USE_SANDBOX_POOL[index].clone();

            // Lock and use the selected sandbox
            let mut sandbox = sandbox.lock().await;
            let result = sandbox.call_guest_function_by_name(
                "DereferenceRawNullPointer",
                ReturnType::Void,
                None,
            );

            // Permit is dropped here, returning it to the pool
            match result {
                Ok(_) => Ok::<_, warp::Rejection>("Guest function called inside warm Hyperlight VM.".to_string()),
                Err(err) => Ok(format!("{:?}", err)),
            }
        })
}
