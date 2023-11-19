use axum::response::Html;
use crate::quotes_struct::Quote;

pub fn view_html(quotes: Vec<Quote>) -> Html<String> {
    let mut data = String::from("<!DOCTYPE html>
    <html>
        <head>
            <style>
                td { border: 1px solid black }
            </style>
        </head>
        <body>
            <table>
                <tr>
                    <th>Author</th>
                    <th>Quote</th>
                </tr>");
    
    for q in quotes {
        data.push_str(&format!("<tr>
            <td>{}</td>
            <td>{}</td>
        </tr>", q.author, q.quote));
    };
    data.push_str("</table></body></html>");
    Html(data)
}