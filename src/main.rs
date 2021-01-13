#[macro_use]
extern crate log;

use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::core::v1::Event;
use kube::{api::{Api, ListParams}, Client};
use kube_runtime::{utils::try_flatten_applied, watcher};
//use rouille::Response;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
//     let _health_check = std::thread::spawn(|| {
//         rouille::start_server("0.0.0.0:4242", move |_request| {
//             Response::text("ok")
//         });
//     });

    std::env::set_var("RUST_LOG", "info,kube=debug");
    env_logger::init();
    let client = Client::try_default().await?;

    let events: Api<Event> = Api::all(client);
    let lp = ListParams::default();

    let mut ew = try_flatten_applied(watcher(events, lp)).boxed();

    while let Some(event) = ew.try_next().await? {
        handle_event(event)?;
    }
    Ok(())
}


// This function lets the app handle an added/modified event from k8s
fn handle_event(ev: Event) -> anyhow::Result<()> {
    //println!("{:#?}", ev);
    match ev.type_.unwrap().as_str() {
        "Normal" => info!(
            "|{}| {} (via {} {})",
            ev.involved_object.namespace.unwrap_or("".to_string()),
            ev.message.unwrap(),
            ev.involved_object.kind.unwrap(),
            ev.involved_object.name.unwrap()
        ),
        "Warning" => error!(
            "|{}| {} (via {} {})",
            ev.involved_object.namespace.unwrap_or("".to_string()),
            ev.message.unwrap(),
            ev.involved_object.kind.unwrap(),
            ev.involved_object.name.unwrap()
        ),
        _ => (),
    }

    Ok(())
}
