use std::collections::VecDeque;

mod agent;
mod puller;
mod errors;

fn main() {
    println!("starting metrico-agent");

    let (tx, rx) =
     std::sync::mpsc::channel::<metriko_common::MetrikoApi>();

    let _puller_thread = std::thread::spawn(move || {
        puller::pull_metrics_from_machine(tx);
    });
    let mut data_queue  = VecDeque::with_capacity(150) ;
    while let Ok(collected_data) = rx.recv() {
        data_queue.push_back(collected_data);
        println!("sending updated metrics to server");
        while  let Some(data) = data_queue.pop_front() {
            if agent::send_data_to_server(&data).is_err() {
                println!("error while sending data to metriko-server");
                data_queue.push_front(data);
                break
            }
        } 
      
    }
}
