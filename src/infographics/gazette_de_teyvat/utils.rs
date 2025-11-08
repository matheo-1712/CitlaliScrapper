fn extract_character_name_gazette(url: &str) -> String {
    // On récupère le dernier segment après le dernier "/"
    let last_segment = url.trim_end_matches('/').split('/').last().unwrap_or("");

    // Si le nom contient des tirets, on garde le dernier fragment
    let clean_name = last_segment.split('-').last().unwrap_or(last_segment);

    clean_name.to_string()
}

fn format_character_name(name: &str) -> String {
    let formatted = name.to_lowercase().replace(" ", "-");
    formatted
        .split('-')
        .last()
        .unwrap_or(&formatted)
        .to_string()
}
