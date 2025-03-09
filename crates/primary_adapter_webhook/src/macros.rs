//TODO: do I need more details here? Normally I should use that only where it is safe to unwrap
#[macro_export]
macro_rules! unwrap_or_webhook_err {
    ($expr:expr) => {
        $expr.map_err(|e| WebhookError::UnwrapError(format!("{} at {}:{}", e, file!(), line!())))?
    };
}

#[macro_export]
macro_rules! option_or_webhook_err {
    ($expr:expr) => {
        $expr.ok_or_else(|| {
            WebhookError::UnwrapError(format!("Missing value at {}:{}", file!(), line!()))
        })?
    };
}
