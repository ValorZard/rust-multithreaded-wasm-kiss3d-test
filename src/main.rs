use kiss3d::prelude::*;
#[cfg(target_arch = "wasm32")]
use kiss3d::wasm_bindgen_futures::spawn_local;
#[cfg(not(target_arch = "wasm32"))]
use std::thread;

mod connection;

#[kiss3d::main]
async fn main() {
    let mut window = Window::new("Kiss3d: cube").await;
    let mut camera = OrbitCamera3d::default();
    let mut scene = SceneNode3d::empty();
    scene
        .add_light(Light::point(100.0))
        .set_position(Vec3::new(0.0, 2.0, -2.0));

    let mut c = scene.add_cube(1.0, 1.0, 1.0).set_color(RED);

    let rot = Quat::from_axis_angle(Vec3::Y, 0.014);

    // We don't even need to call this code for it to not build
    //#[cfg(target_arch = "wasm32")]
    //wasm_safe_thread::spawn(crate::connection::connect_to_server_wasm);

    #[cfg(not(target_arch = "wasm32"))]
    thread::spawn(crate::connection::connect_to_server_native);

    while window.render_3d(&mut scene, &mut camera).await {
        c.rotate(rot);
    }
}
