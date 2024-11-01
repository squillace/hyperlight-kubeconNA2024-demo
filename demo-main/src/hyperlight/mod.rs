use hyperlight_host::sandbox_state::sandbox::EvolvableSandbox;
use hyperlight_host::sandbox_state::transition::Noop;
use hyperlight_host::{MultiUseSandbox, UninitializedSandbox};
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use tokio::sync::{Mutex, Semaphore};

pub(super) mod hello_world;
pub(super) mod safety;

const POOL_SIZE: usize = 10;
static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
struct SandboxError;
impl warp::reject::Reject for SandboxError {}

pub(crate) static SANDBOX_POOL: Lazy<Arc<Semaphore>> = Lazy::new(|| Arc::new(Semaphore::new(POOL_SIZE)));

pub(crate) static MULTI_USE_SANDBOX_POOL: Lazy<Vec<Arc<Mutex<MultiUseSandbox>>>> = Lazy::new(|| {
    (0..POOL_SIZE).map(|_| {
        // Make `UninitializedSandbox`
        let uninitialized_sandbox = UninitializedSandbox::new(
            hyperlight_host::GuestBinary::FilePath(crate::DEMO_GUEST_PATH.to_string()),
            None, // default configuration
            None, // default run options
            None, // default host print function
        )
            .unwrap();

        // Evolve to `MultiUseSandbox` and wrap in Arc<Mutex>
        Arc::new(Mutex::new(
            uninitialized_sandbox.evolve(Noop::default()).unwrap(),
        ))
    }).collect()
});

// Function to warm up each sandbox in the pool
pub(super) async fn warm_up_pool() {
    for sandbox in MULTI_USE_SANDBOX_POOL.iter() {
        let _ = sandbox.lock().await;
        println!("Sandbox instance warmed up.");
    }
}