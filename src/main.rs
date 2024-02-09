mod test_handler;

use utoipa::OpenApi;
use utoipauto::utoipauto;

#[utoipauto(paths = "./src")]
#[derive(OpenApi)]
#[openapi]
struct ApiDoc;

fn main() {
    println!("Hello, world!");
}
