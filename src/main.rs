use hello_wgpu::run;


pub fn main() {
    // pollster is a lightweight alternative to e.g. tokio.
    // We don't need all the complex event loop management and only
    // care about blocking on an future until it completes.
    pollster::block_on(run());
}