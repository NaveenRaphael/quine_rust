fn main() {
    let hash = '#';
    let end = r#";
    println!(
        "fn main() {{
    let hash = '#';
    let end = r{}\"{}\"{}{}
    );
}}",
        hash, 
        end,
        hash,
        end.strip_suffix('"').unwrap().strip_suffix('\\').unwrap()\""#;
    println!(
        "fn main() {{
    let hash = '#';
    let end = r{}\"{}\"{}{}
    );
}}",
        hash,
        end,
        hash,
        end.strip_suffix('"').unwrap().strip_suffix('\\').unwrap()
    );
}
