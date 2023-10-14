use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use winapi::shared::windef::POINT;
use winapi::um::winuser::{GetAsyncKeyState, GetCursorPos, MK_LBUTTON, VK_SHIFT};
use lazy_static::lazy_static;

// 定义一个全局的原子布尔变量来表示 Shift 键的状态
lazy_static! {
    static ref SHIFT_PRESSED: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

fn main() {
    // 监听 Shift 键的状态
    let shift_pressed_clone = SHIFT_PRESSED.clone();
    thread::spawn(move || {
        loop {
            // 获取 Shift 键的状态
            let shift_state = unsafe { GetAsyncKeyState(VK_SHIFT) };
            let is_shift_pressed = shift_state & 0x8000 != 0;

            // 更新原子变量的状态
            shift_pressed_clone.store(is_shift_pressed, Ordering::Relaxed);
        }
    });

    // 监听鼠标左键点击事件
    let mut is_previous_shift_pressed = false;
    loop {
        // 获取 Shift 键和鼠标左键的状态
        let shift_pressed_now = SHIFT_PRESSED.load(Ordering::Relaxed);
        let mouse_state = unsafe { GetAsyncKeyState(MK_LBUTTON) };
        let is_mouse_left_clicked = mouse_state & 0x8000 != 0;

        // 判断 Shift 键和鼠标左键的状态
        if shift_pressed_now && !is_previous_shift_pressed && is_mouse_left_clicked {
            // 触发鼠标左键点击30次
            for _ in 0..30 {
                // 模拟鼠标点击事件
                // 这里只是简单演示，并没有使用实际的点击操作
                println!("Mouse left button clicked");
                thread::sleep(std::time::Duration::from_millis(10)); // 可以根据需要调整点击间隔
            }
        }

        // 更新前一次的 Shift 键状态
        is_previous_shift_pressed = shift_pressed_now;

        // 每次循环休眠一小段时间，避免过多消耗 CPU 资源
        thread::sleep(std::time::Duration::from_millis(10));
    }
}
