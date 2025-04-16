use crate::database::Database;
use crate::models::User;
use wasm_bindgen::prelude::*;

pub struct UserDb {
    db: Database,
}

impl UserDb {
    const IDB_VERSION: u32 = 1;
    const STORE_NAME: &str = "adt_sync_users";
    const STORE_KEY: &str = "id";

    pub async fn new() -> Result<Self, JsValue> {
        let db = Database::new(
            Self::STORE_NAME,
            Self::IDB_VERSION,
            Box::new(|db: &web_sys::IdbDatabase| {
                let options = web_sys::IdbObjectStoreParameters::new();
                options.set_key_path(&Self::STORE_KEY.into());
                let _ = db.create_object_store_with_optional_parameters(Self::STORE_NAME, &options);
            }),
        )
        .await?;
        Ok(Self { db })
    }

    pub async fn get_all(&self) -> Result<Vec<User>, JsValue> {
        self.db.get_all(Self::STORE_NAME).await
    }

    pub async fn add(&self, user: &User) -> Result<(), JsValue> {
        self.db.put(Self::STORE_NAME, &user).await
    }

    pub async fn remove(&self, user: &User) -> Result<(), JsValue> {
        self.db.delete(Self::STORE_NAME, &user.id).await
    }
}
