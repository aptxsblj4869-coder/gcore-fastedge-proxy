use fastedge::http::{Request, Response, Body};

#[fastedge::http_handler]
pub fn main(req: Request<Body>) -> std::result::Result<Response<Body>, Box<dyn std::error::Error>> {
    // 检查是否为 WebSocket
    if let Some(v) = req.headers().get("upgrade") {
        if v == "websocket" {
            return Ok(Response::builder()
                .status(101)
                .header("Connection", "Upgrade")
                .header("Upgrade", "websocket")
                .body(Body::empty())?);
        }
    }

    Ok(Response::builder()
        .status(200)
        .body(Body::from("Gcore Node Active"))?)
}
