//! Functions in this module run in a Hyperlight VM and do the following:
//! 1. Dispatch a guest function call (i.e., `PrintOutput`) to the Hyperlight VM.
//! 2. Because the guest function does not have access to any syscalls due to it
//! running with no OS, to print to the console, it must call a host function—which
//! is done via a VMExit. Upon completion of the host function, we re-enter the VM.
//! Then, the VM can obtain the result from the host function call from shared memory.
//! 3. Finally, exit the VM and return the result to the client.

use warp::Filter;

use hyperlight_common::flatbuffer_wrappers::function_types::{ParameterValue, ReturnType};
use hyperlight_host::sandbox_state::sandbox::EvolvableSandbox;
use hyperlight_host::sandbox_state::transition::Noop;
use hyperlight_host::{MultiUseSandbox, UninitializedSandbox};

use crate::hyperlight::{SandboxError, COUNTER, MULTI_USE_SANDBOX_POOL, POOL_SIZE, SANDBOX_POOL};

pub(crate) fn cold() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path("hyperlight")
        .and(warp::path("hello-world"))
        .and(warp::path("cold"))
        .map(move || {
            // Create a new sandbox per request
            let uninitialized_sandbox = UninitializedSandbox::new(
                hyperlight_host::GuestBinary::FilePath(crate::DEMO_GUEST_PATH.to_string()),
                None, // default configuration
                None, // default run options
                None, // default host print function
            )
                .unwrap();
            let mut multi_use_sandbox: MultiUseSandbox =
                uninitialized_sandbox.evolve(Noop::default()).unwrap();

            let message = "Hello, World! I am executing inside of a VM :)\n".to_string();
            multi_use_sandbox
                .call_guest_function_by_name(
                    "PrintOutput",
                    ReturnType::Int,
                    Some(vec![ParameterValue::String(message.clone())]),
                )
                .unwrap();

            "Guest function called inside cold Hyperlight VM.".to_string()
        })
}

use std::sync::atomic::Ordering;

pub(crate) fn warm() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path("hyperlight")
        .and(warp::path("hello-world"))
        .and(warp::path("warm"))
        .and_then(move || async {
            let message = "Hello, World! I am executing inside of a VM :)\n".to_string();

            // Acquire a permit to access the pool
            let _permit = SANDBOX_POOL.acquire().await.unwrap();

            // Select a sandbox based on the round-robin counter
            let index = COUNTER.fetch_add(1, Ordering::SeqCst) % POOL_SIZE;
            let sandbox = MULTI_USE_SANDBOX_POOL[index].clone();

            // Lock and use the selected sandbox
            let mut sandbox = sandbox.lock().await;
            sandbox
                .call_guest_function_by_name(
                    "PrintOutput",
                    ReturnType::Int,
                    Some(vec![ParameterValue::String(message.clone())]),
                )
                .map_err(|_| warp::reject::custom(SandboxError))?;

            // Permit is dropped here, returning it to the pool
            Ok::<_, warp::Rejection>("Guest function called inside warm Hyperlight VM.".to_string())
        })
}
