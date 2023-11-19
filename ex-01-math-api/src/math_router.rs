use axum::extract::Path;

pub async fn math_route(Path((a, b)): Path<(i32, i32)>) -> String {
    let string_ans: String = (a + b).to_string();
    string_ans
}