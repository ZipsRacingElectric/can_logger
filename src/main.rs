use socketcan::{CanSocket, Socket, frame::CanFrame};



fn main() {
    let mut _can_socket_tx = CanSocket::open("vcan0").unwrap();
    
    loop {
        // TODO: Read the can frame data from can_read
        let can_read = _can_socket_tx.read_frame().unwrap();
        // let can_read_frame = CanFrame::Data((can_read.));

        println!("{:?}", can_read);
    }
    

}