use fastedge::{Body, Result};
// 使用 Gcore 编译器推荐的直接导入路径
use fastedge::http::{Request, Response};

#[no_mangle]
pub fn main(req: Request<Body>) -> Result<Response<Body>> {
    // 检查 WebSocket
    if let Some(upgrade) = req.headers().get("upgrade") {
        if upgrade == "websocket" {
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
