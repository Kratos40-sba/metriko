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
        // encoding here instead of encoding each message 
        let encoded_data = metriko_common::encode(&collected_data);
        data_queue.push_back(encoded_data);
        if agent::send_queque_to_server(&mut data_queue).is_err(){
            println!("error while dialing the server - queueing data until the server is up")
        }
      
       /*
       
        data_queue.push_back(encoded_data);
        println!("sending updated metrics to server");
        while  let Some(data) = data_queue.pop_front() {
            if agent::send_queque_to_server(&mut data_queue).is_err() {
                println!("error while sending data to metriko-server");
                data_queue.push_front(data);
                break
            }
        } 
       
        */
      
    }
}
