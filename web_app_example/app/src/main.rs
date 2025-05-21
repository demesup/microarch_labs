use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use serde::Deserialize;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[derive(Deserialize)]
struct MoveQuery {
    dir: String,
}

#[derive(Deserialize)]
struct DisplayQuery {
    text: String,
}

const PICO_IP_PORT: &str = "192.168.166.16:6000";

async fn send_raw_to_pico(message: &str) {
    match TcpStream::connect(PICO_IP_PORT).await {
        Ok(mut stream) => {
            if let Err(e) = stream.write_all(message.as_bytes()).await {
                eprintln!("Failed to send data: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to Pico: {}", e);
        }
    }
}

async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Pico Robot Control</title>
            <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0-beta3/css/all.min.css">
            <style>
                body {
                    font-family: sans-serif;
                    background: linear-gradient(135deg, #6e7dff, #ff6a00);
                    height: 100vh;
                    margin: 0;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    flex-direction: column;
                    color: white;
                    text-align: center;
                }
                h1 {
                    font-size: 3em;
                    margin-bottom: 20px;
                }
                .joystick button {
                    font-size: 1.5em;
                    padding: 15px 30px;
                    margin: 10px;
                    background: rgba(0, 0, 0, 0.6);
                    color: white;
                    border: none;
                    border-radius: 10px;
                    cursor: pointer;
                    transition: background 0.3s ease;
                }
                .joystick button:hover {
                    background: rgba(0, 0, 0, 0.8);
                }
                .joystick button:active {
                    background: rgba(0, 0, 0, 1);
                }
                #display-form input {
                    padding: 10px;
                    width: 80%;
                    font-size: 1.2em;
                    background: rgba(255, 255, 255, 0.5);
                    border: 2px solid #fff;
                    border-radius: 5px;
                    color: white;
                    margin-bottom: 15px;
                }
                #display-form button {
                    padding: 12px 20px;
                    font-size: 1.2em;
                    background: #ff6a00;
                    color: white;
                    border: none;
                    border-radius: 5px;
                    cursor: pointer;
                    transition: background 0.3s ease;
                }
                #display-form button:hover {
                    background: #e55c00;
                }
            </style>
        </head>
        <body>
            <h1>Pico Robot Control</h1>
            <div class="joystick">
                <button onclick="sendMove('forward')">&#8593;</button><br>
                <button onclick="sendMove('left')">&#8592;</button>
                <button onclick="sendMove('right')">&#8594;</button><br>
                <button onclick="sendMove('backward')">&#8595;</button><br>
                <button onclick="sendMove('stop')">Stop</button><br>
                <button onclick="sendMove('turn')">Turn Around</button><br>
            </div>
            <form id="display-form" onsubmit="sendDisplay(); return false;">
                <input type="text" id="display-text" placeholder="Enter text to display" />
                <button type="submit">Send to Display</button>
            </form>
            <script>
                function sendMove(direction) {
                    fetch(`/move?dir=${direction}`);
                }
                function sendDisplay() {
                    const text = document.getElementById("display-text").value;
                    fetch(`/display?text=${encodeURIComponent(text)}`);
                }
            </script>
        </body>
        </html>
    "#)
}

async fn move_handler(query: web::Query<MoveQuery>) -> impl Responder {
    println!("Requested move: {}", query.dir);

    let msg = match query.dir.as_str() {
        "forward" | "left" | "right" | "backward" | "stop" | "turn" => {
            format!("direction:{}", query.dir)
        }
        _ => return HttpResponse::BadRequest().body("Invalid move command"),
    };

    send_raw_to_pico(&msg).await;
    HttpResponse::Ok().body(format!("Command sent: {}", msg))
}

async fn display_handler(query: web::Query<DisplayQuery>) -> impl Responder {
    println!("Display text: {}", query.text);
    let msg = format!("text:'{}'", query.text);
    send_raw_to_pico(&msg).await;
    HttpResponse::Ok().body("Text sent to display")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route("/move", web::get().to(move_handler))
            .route("/display", web::get().to(display_handler))
    })
        .bind("127.0.0.1:8091")?
        .run()
        .await
}
