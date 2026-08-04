//! Configure the host shared-library output for the Node-API addon.

fn main() {
    if std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() != Ok("wasm32") {
        napi_build::setup();
    }
}
