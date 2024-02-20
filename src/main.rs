use std::net::TcpListener;
mod connection_handler;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        connection_handler::handle_connection(stream);
    }
}
