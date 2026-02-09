use fastedge::{http, Body, Request, Response, Result};

#[fastedge::http_handler]
pub fn main(req: Request<Body>) -> Result<Response<Body>> {
    if let Some(upgrade) = req.headers().get("upgrade") {
        if upgrade == "websocket" {
            return Ok(Response::builder()
                .status(101)
                .header("Connection", "Upgrade")
                .header("Upgrade", "websocket")
                .body(Body::empty())?);
        }
    }
    Ok(Response::builder().status(200).body("Active".into())?)
}
