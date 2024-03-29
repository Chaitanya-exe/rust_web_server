use std::net::TcpListener;
use web_server::handle_connection;
use web_server::ThreadPool;
fn main(){
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming(){
        
        let stream = stream.unwrap();
        handle_connection(stream); 
    }
}
