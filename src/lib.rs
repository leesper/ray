pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// design: tcp echo server:
// use log::{info};
// let server = TcpServer::new()
// .bind(1234)
// .onConnect(|conn: &Conn| {
//      info!("{} connected", conn);
// })
// .onData(|conn: &mut Conn, buf: &Buffer| {
//      info!("received: {}", but.to_string());
//      conn.send(buf.to_string());
// });
// server.serve();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
