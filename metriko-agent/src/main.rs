mod agent;
mod puller;

fn main() {
    println!("starting metrico-agent");
    let (tx , rx) = std::sync::mpsc::
    channel::<metriko_common::MetrikoApi>();
    let _puller_thread = std::thread::spawn(move || {
        puller::pull_metrics_from_machine(tx);
    });
    while  let Ok(collected_data) = rx.recv() {
            println!("sending updated metrics to server");
            agent::send_data_to_server(collected_data);
    }
}
