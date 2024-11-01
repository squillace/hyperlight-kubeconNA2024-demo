use warp::Filter;

pub(super) fn hello_world() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path("hello-world").map(move || {
        println!("Hello, World! I am NOT executing inside of a VM :(");

        "Function called outside Hyperlight VM."
    })
}
