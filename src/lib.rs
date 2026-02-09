// 别再用那些报错的导入了，直接用编译器提示的路径
use fastedge::body::Body;
use fastedge::http::{Request, Response};

// 放弃那个报 E0433 错误的宏 #[fastedge::http_handler]
// 直接用 Rust 最原始的导出方式，这是最稳的
#[no_mangle]
pub fn main(req: Request<Body>) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    // 检查 WebSocket (这是你最关心的逻辑)
    if let Some(upgrade) = req.headers().get("upgrade") {
        if upgrade == "websocket" {
            return Ok(Response::builder()
                .status(101)
                .header("Connection", "Upgrade")
                .header("Upgrade", "websocket")
                .body(Body::empty())?);
        }
    }

    // 默认返回，证明节点在线
    Ok(Response::builder()
        .status(200)
        .body(Body::from("Gcore Edge Node: Active"))?)
}
