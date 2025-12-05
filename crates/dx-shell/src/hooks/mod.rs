pub fn bash_hook() -> &'static str {
    r#"dx_hook() {
    local cmd="$1"
    eval "$cmd"
}
"#
}
