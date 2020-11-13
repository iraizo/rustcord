/*
    Usage: wrapper around notifica
*/

pub mod notifications {
    use notify_rust::Notification;

    pub fn send(summary: &str, body: &str, icon: &str) {
        Notification::new()
            .summary(summary)
            .body(body)
            .icon(icon)
            .show();
    }
}
