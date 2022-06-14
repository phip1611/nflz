use nflz::NFLZAssistant;

/// Minimal example that renames all files in the given directory.
/// After the operation is done, all will include the same amount of digits
/// inside their number group inside the filename.
fn main() {
    let assistant = NFLZAssistant::new("./test-resources").unwrap();
    dbg!(assistant.files_to_rename());
    // some files may already have the correct name
    dbg!(assistant.files_without_rename());
    if assistant.check_can_rename_all().is_ok() {
        assistant.rename_all().unwrap();
    }
}
