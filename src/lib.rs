use fastedge::{Body, Request, Response, Result};

#[fastedge::http_handler]
pub fn main(req: Request<Body>) -> Result<Response<Body>> {
    // 检查 WebSocket 升级
    let is_ws = req.headers()
        .get("upgrade")
        .map_with_if(|v| v == "websocket", || false);

    if is_ws {
        return Ok(Response::builder()
            .status(101)
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .body(Body::empty())?);
    }

    Ok(Response::builder()
        .status(200)
        .body(Body::from("Gcore Node Active"))?)
}
