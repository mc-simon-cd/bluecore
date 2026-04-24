# 🛠️ BlueCore Teknik Borçlar (Technical Debt)

Bu belge, projenin geliştirilmesi sırasında oluşan teknik borçları, performans darboğazlarını ve dikkat edilmesi gereken noktaları takip etmek amacıyla oluşturulmuştur.

## 🔴 Yüksek Öncelikli (Gecikmemesi Gerekenler)

### 1. Tauri ↔ Chromium Geçiş Gecikmesi (Latency)
* **Sorun:** Motor değiştirme sırasında kullanıcı deneyimini etkileyebilecek bir gecikme yaşanabilir.
* **Hedef:** Geçiş süresini minimize etmek ve "State Migration" (durum taşıma) mekanizmasını optimize etmek.

### 2. NTP Ayarlarının Kalıcılığı (SQLite)
* **Sorun:** Mevcut NTP (Yeni Sekme Sayfası) özelleştirmeleri sadece bellek üzerinde tutuluyor; uygulama kapandığında kaybolur.
* **Hedef:** SQLite entegrasyonu ile arka plan, kısayol ve tema verilerini kalıcı hale getirmek.

## 🟡 Orta Öncelikli (Gelişim Sürecinde Çözülecekler)

### 3. TTS (Speak Content) Optimizasyonu
* **Sorun:** Mecvut TTS API'si sadece bir proksidir (placeholder).
* **Hedef:** OS-native TTS (örneğin Windows SAPI veya Linux Speech-dispatcher) veya harici bir AI model entegrasyonu sağlamak.

### 4. Rust Unsafe Blok Denetimi
* **Sorun:** Rust tarafındaki bellek güvenliğini sağlamak için kullanılan `unsafe` bloklar dikkatle incelenmeli.
* **Hedef:** Periyodik kod incelemeleri (audit) ile `unsafe` kullanımını minimuma indirmek ve olanları belgelemek.

### 4. Modül Konsolidasyonu ve Build Sistemi
* **Sorun:** Proje büyüdükçe Arch Linux ve diğer platformlar için derleme süreleri ve bağımlılık yönetimi karmaşıklaşabilir.
* **Hedef:** Build script'lerini (özellikle Rust/C++ bridge kısımlarını) daha modüler ve hızlı hale getirmek.

## 🟢 Gelecek Notları (Vizyon ve İyileştirmeler)

### 5. Sandboxing ve Güvenlik Sıkılaştırması
* **Not:** Phase 4'te plugin sistemi devreye girdiğinde, üçüncü taraf modüllerin sistem erişimini kısıtlamak için gelişmiş sandbox politikaları uygulanmalı.

### 6. AI ve Gizlilik Modülleri Optimizasyonu
* **Not:** Yapay zeka destekli asistan modülünün yerel (local) çalışması durumunda model ağırlıklarının ve çıkarım (inference) motorunun optimize edilmesi gerekecek.

---
*Son Güncelleme: 24 Nisan 2026*
