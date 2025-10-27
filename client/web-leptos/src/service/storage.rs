use leptos::prelude::window;

pub fn get_local_storage(key: &str) -> Option<String> {
    let storage = match window().local_storage() {
        Ok(Some(storage)) => storage,
        Ok(None) => {
            tracing::warn!("no local storage found");
            return None;
        }
        Err(err) => {
            tracing::error!(error = ?err, "unable to get local storage");
            return None;
        }
    };
    match storage.get(key) {
        Ok(value) => value,
        Err(err) => {
            tracing::error!(error = ?err, "unable to get value from local storage");
            None
        }
    }
}

pub fn set_local_storage(key: &str, value: &str) {
    let storage = match window().local_storage() {
        Ok(Some(storage)) => storage,
        Ok(None) => {
            tracing::warn!("no local storage found");
            return;
        }
        Err(err) => {
            tracing::error!(error = ?err, "unable to get local storage");
            return;
        }
    };
    if let Err(err) = storage.set_item(key, value) {
        tracing::error!(error = ?err, "unable to get value from local storage");
    }
}

pub fn remove_local_storage(key: &str) {
    let storage = match window().local_storage() {
        Ok(Some(storage)) => storage,
        Ok(None) => {
            tracing::warn!("no local storage found");
            return;
        }
        Err(err) => {
            tracing::error!(error = ?err, "unable to get local storage");
            return;
        }
    };

    if let Err(err) = storage.remove_item(key) {
        tracing::error!(error = ?err, "unable to remove value from local storage");
    }
}
