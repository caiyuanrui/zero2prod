use std::net::TcpListener;

use dotenv_codegen::dotenv;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = dotenv!("DEV_PORT");
    let lst = TcpListener::bind(format!("127.0.0.1:{port}"))?;
    run(lst)?.await
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use std::net::TcpListener;
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        println!("{:?}", listener.local_addr());
    }
}
