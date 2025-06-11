#[macro_export]
macro_rules! log_fail {
    ($err:expr, $level:expr, $msg:expr, $module:expr, $user_id:expr, $db:expr) => {{
        let full = format!("{}: {}", $msg, $err);
        crate::logs::service::log_event($level, &full, $module, $user_id, $db).await;
    }};
}
