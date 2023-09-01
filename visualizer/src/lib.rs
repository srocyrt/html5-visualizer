use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(a: JsValue, b: JsValue) {
    let a: f64 = a.as_f64().unwrap();
    let b: f64 = b.as_f64().unwrap();
    let msg = format!("Hello, {} + {} = {}!", a, b, add(a, b));
    alert(&msg);
}

pub fn add(left: f64, right: f64) -> f64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2.0, 2.0);
        assert_eq!(result, 4.0);
    }
}
