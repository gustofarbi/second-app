use std::convert::Infallible;
use serde::{Serialize, Deserialize};
use warp::{Filter, hyper::StatusCode};

// static TEMPLATE: &str = include!("static/index.html");

#[derive(Deserialize, Serialize)]
struct Request {
    name: Option<String>,
    pass: Option<String>,
}

#[tokio::main]
async fn main() {
    // println!("{}", TEMPLATE);
    std::env::set_var("RUST_LOG", "server=info");
    env_logger::init();

    let route = warp::get()
        .and(warp::query())
        .and_then(handler)
        .with(warp::log("server"));

    warp::serve(route).run(([0, 0, 0, 0], 8000)).await;
}

async fn handler(input: Request) -> Result<impl warp::Reply, Infallible> {
    let template = "<html>
<body>
<h1>
    Hello, {name}
</h1>
<p>
    your pass is {pass}
</p>
</body>
</html>";

    let mut renderer = tinytemplate::TinyTemplate::new();
    renderer.add_template("foobar", template).unwrap();

    let res = renderer.render("foobar", &input).unwrap();
    let resp = warp::reply::html(res);
    Ok(warp::reply::with_status(resp, StatusCode::OK))
}