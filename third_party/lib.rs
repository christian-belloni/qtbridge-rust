pub fn register() {
    qtbridge::qresource::register_bytes_with_prefix(
        include_bytes!(std::env!("RCC_FILE")),
        std::env!("PREFIX"),
    );
}
