use fastedge::http::{Request, Response, Body};

// 放弃使用报错的 #[fastedge::http_handler] 宏，改用底层导出方式
#[no_mangle]
pub fn main(req: Request<Body>) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    // 处理 WebSocket 握手
    if let Some(upgrade) = req.headers().get("upgrade") {
        if upgrade == "websocket" {
            return Ok(Response::builder()
                .status(101)
                .header("Connection", "Upgrade")
                .header("Upgrade", "websocket")
                .body(Body::empty())?);
        }
    }

    // 默认返回
    Ok(Response::builder()
        .status(200)
        .body(Body::from("Gcore Node Active"))?)
}
