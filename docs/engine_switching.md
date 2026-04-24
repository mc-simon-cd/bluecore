# Engine Switching Logic
**License:** Apache 2.0 | **Project:** Simon Project | **Son Güncelleme:** 24 Nisan 2026

---

## Genel Akış

```
[Kullanıcı / Tetikleyici]
        │
        ▼ switch_engine("chromium")
[Tauri IPC – engine_api.rs]
        │ engine_manager.lock().await
        ▼
[EngineManager::switch()]
   1. active_engine.shutdown()    → Result<(), String>
   2. active_engine = new_engine  (TauriEngine | ChromiumEngine)
   3. active_engine.boot()        → Result<(), String>
        │
        ▼
[BrowserEngine Trait – engine_trait.rs]
   fn navigate(&self, url: &str) -> Result<(), String>
   fn boot(&mut self)             -> Result<(), String>
   fn shutdown(&mut self)         -> Result<(), String>
```

---

## Desteklenen Motorlar

| Motor | Enum | Dosya |
|---|---|---|
| Tauri Engine | `TauriEngine` | `engines/tauri_engine.rs` |
| Chromium Fork | `ChromiumEngine` | `engines/chromium_fork.rs` |

---

## Aktivasyon Koşulları

Motor değiştirme iki yoldan tetiklenebilir:

1. **Config:** `config/engines.json` içinde `"enabled": true` ayarlamak.
2. **Trigger Dosyası:** `docs/enable_chromium.md` dosyasının `TOKEN: BLC-ENABLE-CHROME-V1` içermesi.

---

## Hata Yönetimi

Tüm engine işlemleri `Result<T, String>` döndürür. Motor başarısız olursa:
- Hata mesajı frontend'e iletilir.
- `EngineManager` önceki durumda askıya alınır (Circuit Breaker planlanmaktadır).

---

## İlgili Dosyalar
- [`src-tauri/src/core/engine_manager.rs`](../src-tauri/src/core/engine_manager.rs)
- [`src-tauri/src/engines/engine_trait.rs`](../src-tauri/src/engines/engine_trait.rs)
- [`src-tauri/src/api/engine_api.rs`](../src-tauri/src/api/engine_api.rs)
- [`config/engines.json`](../config/engines.json)
