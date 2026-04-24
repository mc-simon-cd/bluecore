# BlueCore Module System Design
**License:** Apache 2.0 | **Project:** Simon Project | **Son Güncelleme:** 24 Nisan 2026

---

## Genel Bakış

BlueCore modülleri, tarayıcıya çekirdek dışı özellikler ekleyen bağımsız bileşenlerdir. Her modül, `BlueCoreModule` trait'ini implement ederek sisteme entegre olur.

---

## Dizin Yapısı

```
/src-tauri/src/modules/        → Backend modülleri (Rust)
  ├── mod.rs                   → BlueCoreModule trait + ModuleRegistry
  └── privacy_shield.rs        → PrivacyShield implementasyonu

/src/app/modules/              → Frontend modülleri (React)
  ├── registry.ts              → İstemci tarafı modül kaydı
  └── ...
```

---

## BlueCoreModule Trait

```rust
#[async_trait]
pub trait BlueCoreModule: Send + Sync {
    fn metadata(&self) -> ModuleMetadata;
    async fn on_initialize(&self) -> Result<(), String>;
    async fn on_message(&self, action: String, data: Value) -> Result<Value, String>;
}
```

---

## ModuleRegistry

Modüller merkezi bir `ModuleRegistry` üzerinden yönetilir. Registry, `RwLock` ile thread-safe hale getirilmiştir:

```rust
pub struct ModuleRegistry {
    modules: Arc<RwLock<HashMap<String, Box<dyn BlueCoreModule>>>>,
}
```

---

## Mevcut Modüller

| Modül | ID | Açıklama |
|---|---|---|
| **PrivacyShield** | `privacy-shield` | Tracker ve reklam engelleme |

---

## Kurallar

- Modüller direkt motor erişimi **YAPAMAZ**.
- Tüm iletişim `BlueCoreAPI` üzerinden geçmelidir.
- Modül aksiyonları `execute_module_action(id, action, data)` komutu ile çağrılır.

---

## İlgili Dosyalar
- [`src-tauri/src/modules/mod.rs`](../src-tauri/src/modules/mod.rs)
- [`src-tauri/src/api/modules.rs`](../src-tauri/src/api/modules.rs)
