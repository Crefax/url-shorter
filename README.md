# URL Kısaltma Servisi

Bu proje, uzun URL'leri kısa ve kullanışlı URL'lere dönüştüren bir web servisidir. Ve Tamamen Cursur AI İle Yapılmıştır.

## Özellikler

- Basit ve kullanıcı dostu arayüz
- URL kısaltma işlemi
- 6 karakterli benzersiz kısa kodlar (A-Z, a-z, 0-9)
- Yönlendirme sistemi
- Tıklanma sayısı takibi

## Teknik Detaylar

### Backend
- Rust programlama dili
- Actix web framework
- MongoDB veritabanı

### Veritabanı Şeması
```
{
    URL: String,        // Orijinal uzun URL
    ShortCode: String,  // Kısaltılmış benzersiz kod (6 karakter)
    Clicks: i32         // Tıklanma sayısı
}
```

## Kurulum

1. Rust'ı yükleyin
2. MongoDB'yi yükleyin
3. Projeyi klonlayın
4. Bağımlılıkları yükleyin
5. Projeyi çalıştırın

## Geliştirme

Projeyi geliştirmek için:

```bash
cargo run
```

## Lisans

MIT 