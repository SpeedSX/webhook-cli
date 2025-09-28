use chrono::DateTime;
use colored::Colorize;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

use crate::models::WebhookRequest;

pub fn print_request_summary(request: &WebhookRequest) {
    let time = format_date(&request.date);
    let method = format_method(&request.message_object.method);
    let path = extract_path(&request.message_object.value, &request.token_id);

    println!(
        "{} {} {} {} {}",
        time.bright_black(),
        method,
        path.bright_white(),
        format!("({})", request.id).bright_black(),
        get_body_preview(&request.message_object.body).bright_yellow()
    );
}

pub fn print_request_body(request: &WebhookRequest) {
    if let Some(body) = &request.message_object.body {
        if !body.trim().is_empty() {
            println!(
                "{}: {}",
                "Body".bright_blue().bold(),
                if body.len() > 200 {
                    format!("{}...", &body[..200])
                } else {
                    body.clone()
                }
                .bright_white()
            );
        }
    }
}

pub fn print_request_headers(request: &WebhookRequest) {
    if !request.message_object.headers.is_empty() {
        println!("{}", "HEADERS".bright_cyan().bold());
        for (key, values) in &request.message_object.headers {
            for value in values {
                println!("  {}: {}", key.bright_blue(), value.bright_white());
            }
        }
    }
}

pub fn print_full_request_body(request: &WebhookRequest) {
    println!("{}", "REQUEST BODY".bright_cyan().bold());
    println!("{}", "─".repeat(30).bright_black());

    if let Some(body) = &request.message_object.body {
        if body.trim().is_empty() {
            println!("{}", "(empty)".bright_black());
        } else {
            // Try to pretty-print JSON with syntax highlighting
            match serde_json::from_str::<serde_json::Value>(body) {
                Ok(json) => {
                    let pretty_json = serde_json::to_string_pretty(&json).unwrap();
                    highlight_json(&pretty_json);
                    println!(); // Add newline after the highlighted JSON
                }
                Err(_) => {
                    // Not JSON, check if it's form data or other structured format
                    if body.contains('&')
                        && (body.contains('=')
                            || body.starts_with("application/x-www-form-urlencoded"))
                    {
                        // Try to format form data nicely
                        println!("{}", format_form_data(body).bright_white());
                    } else {
                        // Raw text with proper line breaks
                        println!("{}", body.bright_white());
                    }
                }
            }
        }
    } else {
        println!("{}", "(no body)".bright_black());
    }
}

pub fn print_request_details(request: &WebhookRequest) {
    println!("{}", "REQUEST DETAILS".bright_green().bold());
    println!("{}", "═".repeat(50).bright_black());

    // Basic info
    println!(
        "{}: {}",
        "ID".bright_blue().bold(),
        request.id.bright_white()
    );
    println!(
        "{}: {}",
        "Token".bright_blue().bold(),
        request.token_id.bright_white()
    );
    println!(
        "{}: {}",
        "Date".bright_blue().bold(),
        format_date(&request.date).bright_white()
    );
    println!(
        "{}: {}",
        "Method".bright_blue().bold(),
        format_method(&request.message_object.method)
    );
    println!(
        "{}: {}",
        "Path".bright_blue().bold(),
        request.message_object.value.bright_white()
    );
    println!();

    // Headers
    println!("{}", "HEADERS".bright_cyan().bold());
    println!("{}", "─".repeat(30).bright_black());
    for (key, values) in &request.message_object.headers {
        for value in values {
            println!("{}: {}", key.bright_blue(), value.bright_white());
        }
    }
    println!();

    // Query Parameters
    if !request.message_object.query_parameters.is_empty() {
        println!("{}", "QUERY PARAMETERS".bright_cyan().bold());
        println!("{}", "─".repeat(30).bright_black());
        for param in &request.message_object.query_parameters {
            println!("{}", param.bright_white());
        }
        println!();
    }

    // Body
    println!("{}", "REQUEST BODY".bright_cyan().bold());
    println!("{}", "─".repeat(30).bright_black());
    if let Some(body) = &request.message_object.body {
        if body.trim().is_empty() {
            println!("{}", "(empty)".bright_black());
        } else {
            // Try to pretty-print JSON with syntax highlighting
            match serde_json::from_str::<serde_json::Value>(body) {
                Ok(json) => {
                    let pretty_json = serde_json::to_string_pretty(&json).unwrap();
                    highlight_json(&pretty_json);
                    println!(); // Add newline after the highlighted JSON
                }
                Err(_) => {
                    println!("{}", body.bright_white());
                }
            }
        }
    } else {
        println!("{}", "(no body)".bright_black());
    }
}

pub fn highlight_json(json: &str) {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps
        .find_syntax_by_extension("json")
        .or_else(|| ps.find_syntax_by_name("JSON"))
        .unwrap_or_else(|| ps.find_syntax_plain_text());

    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    for line in LinesWithEndings::from(json) {
        let ranges: Vec<(syntect::highlighting::Style, &str)> =
            h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        print!("{}", escaped);
    }
}

pub fn format_form_data(data: &str) -> String {
    data.split('&')
        .map(|pair| {
            if let Some((key, value)) = pair.split_once('=') {
                format!(
                    "{}: {}",
                    urlencoding::decode(key).unwrap_or_else(|_| key.into()),
                    urlencoding::decode(value).unwrap_or_else(|_| value.into())
                )
            } else {
                pair.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn format_method(method: &str) -> colored::ColoredString {
    match method.to_uppercase().as_str() {
        "GET" => method.green().bold(),
        "POST" => method.blue().bold(),
        "PUT" => method.yellow().bold(),
        "DELETE" => method.red().bold(),
        "PATCH" => method.magenta().bold(),
        _ => method.white().bold(),
    }
}

pub fn format_date(date_str: &str) -> String {
    match DateTime::parse_from_rfc3339(date_str) {
        Ok(dt) => dt.format("%H:%M:%S").to_string(),
        Err(_) => date_str.to_string(),
    }
}

pub fn extract_path(full_path: &str, token: &str) -> String {
    if let Some(token_index) = full_path.find(token) {
        let after_token = &full_path[token_index + token.len()..];
        if after_token.is_empty() {
            "/".to_string()
        } else {
            after_token.to_string()
        }
    } else {
        full_path.to_string()
    }
}

pub fn get_body_preview(body: &Option<String>) -> String {
    match body {
        Some(b) if !b.trim().is_empty() => {
            let trimmed = b.trim();
            let mut preview: String = trimmed.chars().take(50).collect();
            if trimmed.chars().count() > 50 {
                preview.push('…');
            }
            format!("[BODY] {}", preview)
        }
        _ => "[BODY] (empty)".to_string(),
    }
}