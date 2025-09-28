use futures_util::{SinkExt, StreamExt};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode, body::Incoming as IncomingBody};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

// WebSocket 연결을 처리하는 함수
async fn handle_websocket(stream: TcpStream, addr: std::net::SocketAddr) {
    println!("New WebSocket connection from: {}", addr);

    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            println!("Error accepting WebSocket connection: {}", e);
            return;
        }
    };

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // 환영 메시지 전송
    if let Err(e) = ws_sender
        .send(Message::Text(
            "Welcome to WebSocket echo server!".to_string(),
        ))
        .await
    {
        println!("Error sending welcome message: {}", e);
        return;
    }

    // 클라이언트로부터 메시지를 받아서 에코
    while let Some(message) = ws_receiver.next().await {
        match message {
            Ok(msg) => {
                match msg {
                    Message::Text(text) => {
                        println!("Received text: {}", text);
                        let echo_msg = format!("Echo: {}", text);
                        if let Err(e) = ws_sender.send(Message::Text(echo_msg)).await {
                            println!("Error sending echo message: {}", e);
                            break;
                        }
                    }
                    Message::Binary(data) => {
                        println!("Received binary data: {} bytes", data.len());
                        if let Err(e) = ws_sender.send(Message::Binary(data)).await {
                            println!("Error sending binary message: {}", e);
                            break;
                        }
                    }
                    Message::Close(_) => {
                        println!("WebSocket connection closed by client");
                        break;
                    }
                    Message::Ping(data) => {
                        println!("Received ping");
                        if let Err(e) = ws_sender.send(Message::Pong(data)).await {
                            println!("Error sending pong: {}", e);
                            break;
                        }
                    }
                    Message::Pong(_) => {
                        println!("Received pong");
                    }
                    Message::Frame(_) => {
                        // Raw frame, usually handled internally
                    }
                }
            }
            Err(e) => {
                println!("Error receiving message: {}", e);
                break;
            }
        }
    }

    println!("WebSocket connection from {} closed", addr);
}

// HTTP 요청을 처리하는 함수
async fn handle_request(_req: Request<IncomingBody>) -> Result<Response<Full<Bytes>>, Infallible> {
    // HTML 페이지 반환
    let html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>WebSocket Test</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        #messages { 
            border: 1px solid #ccc; 
            height: 300px; 
            overflow-y: scroll; 
            padding: 10px; 
            margin: 10px 0;
            background-color: #f9f9f9;
        }
        input, button { 
            margin: 5px; 
            padding: 8px;
        }
        input[type="text"] {
            width: 300px;
        }
    </style>
</head>
<body>
    <h1>WebSocket Echo Server Test</h1>
    <div>
        <button onclick="connect()">Connect</button>
        <button onclick="disconnect()">Disconnect</button>
        <span id="status">Disconnected</span>
    </div>
    
    <div id="messages"></div>
    
    <div>
        <input type="text" id="messageInput" placeholder="메시지를 입력하세요">
        <button onclick="sendMessage()">Send</button>
    </div>
    
    <script>
        let socket = null;
        const messages = document.getElementById('messages');
        const status = document.getElementById('status');
        
        function connect() {
            if (socket && socket.readyState === WebSocket.OPEN) {
                addMessage('Already connected');
                return;
            }
            
            socket = new WebSocket('ws://localhost:8083/');
            
            socket.onopen = function(event) {
                addMessage('✅ Connected to WebSocket server');
                status.textContent = 'Connected';
                status.style.color = 'green';
            };
            
            socket.onmessage = function(event) {
                addMessage('📨 Received: ' + event.data);
            };
            
            socket.onclose = function(event) {
                addMessage('❌ WebSocket connection closed');
                status.textContent = 'Disconnected';
                status.style.color = 'red';
            };
            
            socket.onerror = function(error) {
                addMessage('💥 WebSocket error: ' + error);
                status.textContent = 'Error';
                status.style.color = 'red';
            };
        }
        
        function disconnect() {
            if (socket) {
                socket.close();
            }
        }
        
        function sendMessage() {
            const input = document.getElementById('messageInput');
            if (socket && socket.readyState === WebSocket.OPEN) {
                socket.send(input.value);
                addMessage('📤 Sent: ' + input.value);
                input.value = '';
            } else {
                addMessage('❌ WebSocket is not connected');
            }
        }
        
        function addMessage(message) {
            const div = document.createElement('div');
            div.textContent = new Date().toLocaleTimeString() + ' - ' + message;
            messages.appendChild(div);
            messages.scrollTop = messages.scrollHeight;
        }
        
        document.getElementById('messageInput').addEventListener('keypress', function(e) {
            if (e.key === 'Enter') {
                sendMessage();
            }
        });
        
        // 자동 연결
        window.addEventListener('load', function() {
            addMessage('🌐 Page loaded. Click Connect to start WebSocket connection.');
        });
    </script>
</body>
</html>
"#;

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html; charset=utf-8")
        .body(Full::new(Bytes::from(html)))
        .unwrap();
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting servers...");
    println!("HTTP server: http://localhost:8082");
    println!("WebSocket server: ws://localhost:8083");

    // HTTP 서버를 별도 포트에서 실행
    let http_listener = TcpListener::bind("127.0.0.1:8082").await?;
    tokio::spawn(async move {
        while let Ok((stream, _addr)) = http_listener.accept().await {
            let io = TokioIo::new(stream);
            tokio::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service_fn(handle_request))
                    .await
                {
                    println!("Error serving HTTP connection: {:?}", err);
                }
            });
        }
    });

    // WebSocket 서버를 별도 포트에서 실행
    let ws_listener = TcpListener::bind("127.0.0.1:8083").await?;
    while let Ok((stream, addr)) = ws_listener.accept().await {
        tokio::spawn(handle_websocket(stream, addr));
    }

    Ok(())
}
