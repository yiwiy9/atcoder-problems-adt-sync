use serde::{Serialize, de::DeserializeOwned};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys;

pub struct Database {
    db: web_sys::IdbDatabase,
}

impl Database {
    pub async fn new(
        name: &str,
        version: u32,
        on_upgrade_needed: Box<dyn Fn(&web_sys::IdbDatabase)>,
    ) -> Result<Self, JsValue> {
        // Use `global()` to support environments like Service Workers where `window` is not available.
        let indexed_db_value =
            js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("indexedDB"))?;

        let indexed_db = indexed_db_value
            .dyn_into::<web_sys::IdbFactory>()
            .map_err(|_| JsValue::from_str("Failed to get indexedDB from global object"))?;

        let open_db_request = indexed_db.open_with_u32(name, version)?;

        Self::set_onupgradeneeded(&open_db_request, on_upgrade_needed);

        let db = Self::request_to_future(&open_db_request)
            .await?
            .dyn_into::<web_sys::IdbDatabase>()?;

        Ok(Self { db })
    }

    pub async fn get<K, V>(&self, store_name: &str, key: &K) -> Result<Option<V>, JsValue>
    where
        K: Serialize,
        V: DeserializeOwned,
    {
        let transaction = self.db.transaction_with_str(store_name)?;
        let store = transaction.object_store(store_name)?;

        let get_request = store.get(&serde_wasm_bindgen::to_value(key)?)?;

        let result = Self::request_to_future(&get_request).await?;

        if result.is_undefined() {
            Ok(None)
        } else {
            let value: V = serde_wasm_bindgen::from_value(result)?;
            Ok(Some(value))
        }
    }

    pub async fn get_all<T>(&self, store_name: &str) -> Result<Vec<T>, JsValue>
    where
        T: DeserializeOwned,
    {
        let transaction = self.db.transaction_with_str(store_name)?;
        let store = transaction.object_store(store_name)?;

        let get_all_request = store.get_all()?;

        let values: Vec<T> =
            serde_wasm_bindgen::from_value(Self::request_to_future(&get_all_request).await?)?;

        Ok(values)
    }

    pub async fn put<T>(&self, store_name: &str, value: &T) -> Result<(), JsValue>
    where
        T: Serialize,
    {
        let transaction = self
            .db
            .transaction_with_str_and_mode(store_name, web_sys::IdbTransactionMode::Readwrite)?;
        let store = transaction.object_store(store_name)?;

        let put_request = store.put(&serde_wasm_bindgen::to_value(value)?)?;

        Self::request_to_future(&put_request).await?;

        Ok(())
    }

    pub async fn delete<T>(&self, store_name: &str, key: &T) -> Result<(), JsValue>
    where
        T: Serialize,
    {
        let transaction = self
            .db
            .transaction_with_str_and_mode(store_name, web_sys::IdbTransactionMode::Readwrite)?;
        let store = transaction.object_store(store_name)?;

        let delete_request = store.delete(&serde_wasm_bindgen::to_value(key)?)?;

        Self::request_to_future(&delete_request).await?;

        Ok(())
    }

    /// Converts an IdbRequest into a JsFuture by wrapping it in a Promise,
    /// allowing it to be used with async/await.
    fn request_to_future(request: &web_sys::IdbRequest) -> JsFuture {
        let promise = js_sys::Promise::new(&mut |resolve, reject| {
            Self::set_onsuccess(request, resolve);
            Self::set_onerror(request, reject);
        });

        JsFuture::from(promise)
    }

    fn set_onsuccess(request: &web_sys::IdbRequest, resolve: js_sys::Function) {
        let success_closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            if let Some(result) = event
                .target()
                .and_then(|target| target.dyn_into::<web_sys::IdbRequest>().ok())
                .and_then(|request| request.result().ok())
            {
                resolve.call1(&JsValue::null(), &result).unwrap();
            }
        }) as Box<dyn Fn(_)>);

        request.set_onsuccess(Some(success_closure.into_js_value().unchecked_ref()));
    }

    fn set_onerror(request: &web_sys::IdbRequest, reject: js_sys::Function) {
        let error_closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let error_message = event
                .target()
                .and_then(|target| target.dyn_into::<web_sys::IdbRequest>().ok())
                .and_then(|request| request.error().ok().flatten())
                .map(|error| JsValue::from_str(&error.message()))
                .unwrap_or_else(|| JsValue::from_str("Unknown error"));
            reject.call1(&JsValue::null(), &error_message).unwrap();
        }) as Box<dyn Fn(_)>);

        request.set_onerror(Some(error_closure.into_js_value().unchecked_ref()));
    }

    // No Promise needed for onupgradeneeded as it runs synchronously as part of the database opening transaction;
    // any errors cause the transaction to abort, failing the entire open request.
    fn set_onupgradeneeded(
        request: &web_sys::IdbOpenDbRequest,
        upgrade_handler: Box<dyn Fn(&web_sys::IdbDatabase)>,
    ) {
        let upgrade_closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            if let Some(db) = event
                .target()
                .and_then(|target| target.dyn_into::<web_sys::IdbRequest>().ok())
                .and_then(|request| request.result().ok())
                .and_then(|result| result.dyn_into::<web_sys::IdbDatabase>().ok())
            {
                upgrade_handler(&db);
            }
        }) as Box<dyn Fn(_)>);

        request.set_onupgradeneeded(Some(upgrade_closure.into_js_value().unchecked_ref()));
    }
}
