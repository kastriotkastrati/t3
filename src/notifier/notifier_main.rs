use notify_rust;

pub fn notify() {
    let _ = notify_rust::Notification::new()
        .summary("Firefox News")
        .body("This will almost look like a real firefox notification.")
        .icon("firefox")
        .show();
}
