use std::net::TcpListener;
mod lib;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        lib::handle_connection(stream);
    }
}
