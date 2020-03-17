## 2020-03-18
Ngelanjutin kemaren, aku sudah bisa return home page.

Untuk buttonnya ternyata generate dulu cuy.

Jadi langkah-langkahnya:
1. Get `oauth_token` dengan cara `POST oauth/request_token`
2. Generate link login untuk tombolnya: `https://api.twitter.com/oauth/authenticate?oauth_token=UPDATE`
3. User click link, redirect ke twitter, proses, terus twitter nge forward ke
   callback.

Masalah: semua web framework di rust async nih? gimana dong?

Aku list dulu deh, mana yg paling popular:

- [actix-web](https://github.com/actix/actix-web), 7.6K Stars
- [gotham](https://github.com/gotham-rs/gotham), 1.6K Stars
- [iron](https://github.com/iron/iron), 5.7K Stars
- [nickel](https://github.com/nickel-org/nickel.rs), 2.7K Stars
- [Rocket](https://github.com/SergioBenitez/Rocket), 9.2K Stars
- [tide](https://github.com/http-rs/tide), 1.8K Stars
- [warp](https://github.com/seanmonstar/warp), 1.9K Stars

dan... ternyata rocket.

Okey.

masalahnya kalau rocket dia harus nightly haha.

kalau dari sini dia mau pake untuk [hyper](https://github.com/SergioBenitez/Rocket/issues/1254#issuecomment-596119979).

Hadeh...

Yaudah berarti pake `warp` aja de.




## 2020-03-17
Sampe ke user authentication, masih bingung mau di pake apa `UserAuthentication`
nya.

Kalau di tweepy dia bisa ambil access tokennya cuy.

Ini [dokumentasi auth](https://developer.twitter.com/en/docs/basics/authentication/overview)
dari twitter.

Kita list dulu scopenya.

- Log in with Twitter
- OAuth 1.0a
- OAuth 2.0 Bearer Token
- Basic authentication

Lory harus support semuanya.

Kita mulai dari [Log in with Twitter](https://developer.twitter.com/en/docs/basics/authentication/guides/log-in-with-twitter).

Apa yang bisa dilakukan lory?

Log in with Twitter cocok untuk websites, iOS, mobile & dekstop apps.

Jadi flownya adalah:

1. User akses halaman yang ada sign in buttonnya
2. Twitter forward ke callback url dengan params
2. Convert request token to access token

Oke.

Berarti aku harus buat web page yg ada html pagenya nih.

Cargo buat example gimana ya?

Disini ada [cara buat examples di Cargo](http://xion.io/post/code/rust-examples.html).

Jadi aku mau buat examples untuk login di twitter.

Buat local webserver yg bisa return HTML tombol login twitter
dan bisa proses callback.

Oke aku mengikuti [guide ini](https://hyper.rs/guides/server/hello-world/).

sekarang bagaimana cara buat endpoint index yang return HTML?
