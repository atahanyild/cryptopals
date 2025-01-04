# Derinlemesin Cryptopals - Modül 1 - Materyal

> haz. [Abdullah Talayhan](https://github.com/bufferhe4d/)

Bu modüldeki amaç kriptografide sık kullandığımız bazı işlemler ve veri yapılarıyla haşır neşir olmak. Ardından bazı temel şifreleme yöntemlerinin implementasyonunu ve analizini yapmış olacağız.

Bu modül [Set 1](https://cryptopals.com/sets/1/)'deki ilk 6 egzersizden oluşuyor.

## [Set 1 - Challenge 1](https://cryptopals.com/sets/1/challenges/1)

Bu egzersiz çeşitli verilerin hem 16 tabanında (hexadecimal veya kısaca hex), hem de 64 tabanında (base64) nasıl yazıldığını öğretmeyi amaçlıyor. İki tabandaki gösterimi öğrendikten sonra birini diğerine çevirmek kolay olacaktır.

### Kaynaklar

- <https://en.wikipedia.org/wiki/Hexadecimal>
- <https://en.wikipedia.org/wiki/Base64>

### Tartışma Konuları

1. Normalde herhangi bir sayıyı 64 tabanında yazabiliriz. Base64 encoding yaparken sayıyı 64 tabanında yazmak dışında ek olarak bir dolgulama (padding) işlemi mevcut. Bunun sebebi ne olabilir?

## [Set 1 - Challenge 2](https://cryptopals.com/sets/1/challenges/2)

Bu egzersizde kriptografide çok sık kullanılan mantıksal operatörlerden biri olan xor operatörünü tanımış olacağız. bu operatörü $\oplus$ ile gösteriyoruz.

Doğruluk tablosu şu şekilde:

| $X$ | $Y$ | $X \oplus Y$ |
| --- | --- | ------------ |
| 0   | 0   | 0            |
| 0   | 1   | 1            |
| 1   | 0   | 1            |
| 1   | 1   | 0            |

### Tartışma Konuları

1. 2 tabanında de toplama işlemi ile $\oplus$ operasyonu arasında bir benzerlik var mı?

1. Rastgele seçilmiş $X$ ve $Y$ değerleri için $\oplus$ operasyonunun sonucunun 1 olma olasılığı kaçtır? Bundan yola çıkılarak 0 olma olasılığı kaçtır?

1. Maddeyi diğer mantıksal operatörler için hesaplayıp karşılaştıralım.

## [Set 1 - Challenge 3](https://cryptopals.com/sets/1/challenges/3)

Burada kaba kuvvet (brute force) saldırılarının çok önemli bir noktası öğretiliyor. Kağıt kalem ile de çözülebilir olmasına rağmen bunun otomatik bir şekilde çözülmesinin istenme sebebi de bu.

### Tartışma Konuları

1. Bir kaba kuvvet saldırısı yaparken, saldırı algoritmasının ne zaman duracağına nasıl karar veririz?

1. Şifreleme algoritmasını çözdükten sonra ortaya çıkacak şeyin nasıl bir yapısı olduğunu bilmemiz yukarıdaki soruyu cevaplamamızda bize nasıl yardımcı olur?

1. Bir metnin İngilizce olduğunu anlamak için nasıl bir metrik kullandınız?

## [Set 1 - Challenge 4](https://cryptopals.com/sets/1/challenges/4)

Yukarıdaki egzersizdeki analizin başka bir uygulaması

### Tartışma Konuları

1. Verilen bir dizi şifrelenmiş metinden, metinin nasıl şifrelendiği hakkında bilgi edinebiliyor olmanın bir dezavantajı var mıdır?

1. (1). Maddedeki durumun mükemmel gizlilik (perfect secrecy) şartlarına etkisi nedir?

## [Set 1 - Challenge 5](https://cryptopals.com/sets/1/challenges/5)

### Kaynaklar

- <https://en.wikipedia.org/wiki/Vigenère_cipher>

### Tartışma Konuları

1. Sistemin Vigenére şifrelemesi ile benzerliği nedir?

## [Set 1 - Challenge 6](https://cryptopals.com/sets/1/challenges/6)

### Kaynaklar

- <https://en.wikipedia.org/wiki/Kasiski_examination>

### Tartışma Konuları

1. Bu tarz bir saldırıyı yapmak herhangi bir yer değiştirme şifrelemesi (substitution cipher) için mümkün olabilir mi?
