use wasm_bindgen::prelude::*;

// 讓 Rust 崩潰時能把錯誤訊息印在瀏覽器 console
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Calculator;

#[wasm_bindgen]
impl Calculator {
    // 接收 JS 的字串，回傳計算結果或錯誤訊息
    // Result<f64, JsError> 會讓 JS 端可以用 try-catch 捕捉錯誤
    pub fn eval(expression: &str) -> Result<f64, JsError> {
        match meval::eval_str(expression) {
            Ok(res) => Ok(res),
            Err(e) => {
                // 將 Rust 的錯誤轉為 JS 的 Error 物件
                Err(JsError::new(&format!("解析錯誤: {}", e)))
            }
        }
    }

    // 示範：簡單的加法 (不涉及字串解析，效能最高)
    pub fn add(a: f64, b: f64) -> f64 {
        a + b
    }
}
