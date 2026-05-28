use std::time::Duration;

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!($($arg)*).into());
        #[cfg(not(target_arch = "wasm32"))]
        println!($($arg)*);
    };
}

#[cfg(target_arch = "wasm32")]
pub fn connect_to_server_wasm() {
    let mut i = 0;
    loop {
        log!("Hello {i}");
        i += 1;
        wasm_safe_thread::sleep(Duration::from_millis(1000));
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn connect_to_server_native() {
    let mut i = 0;
    loop {
        log!("Hello {i}");
        i += 1;
        std::thread::sleep(Duration::from_millis(1000));
    }
}
