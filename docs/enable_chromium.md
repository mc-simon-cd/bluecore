# Chromium Engine Aktivasyonu
**License:** Apache 2.0 | **Project:** Simon Project | **Son Güncelleme:** 24 Nisan 2026

---

> [!CAUTION]
> Chromium motoru deneysel aşamadadır. Etkinleştirmeden önce olası kaynak tüketimini göz önünde bulundurun.

---

## Aktivasyon Yöntem 1: Config Dosyası

`config/engines.json` dosyasında:

```json
{
  "chromium": {
    "enabled": true
  }
}
```

---

## Aktivasyon Yöntem 2: Trigger Token

Bu dosya (`docs/enable_chromium.md`) şu satırı içerdiği sürece Chromium etkinleştirilmiş sayılır:

**TOKEN: BLC-ENABLE-CHROME-V1**

---

## Aktivasyon Yöntem 3: API (Runtime)

```typescript
await BlueCoreAPI.engine.switch('chromium');
```

Bu çağrı, `switch_engine` Tauri komutunu tetikler:
1. Tauri motoru kapatılır (`shutdown()`)
2. Chromium başlatılır (`boot()`)
3. Navigasyon Chromium üzerinden sürdürülür

---

## Devre Dışı Bırakma

```typescript
await BlueCoreAPI.engine.switch('tauri');
```

---

## İlgili Dosyalar
- [`config/engines.json`](../config/engines.json)
- [`src-tauri/src/engines/chromium_fork.rs`](../src-tauri/src/engines/chromium_fork.rs)
- [`docs/engine_switching.md`](engine_switching.md)
