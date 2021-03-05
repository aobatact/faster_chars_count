Library for counting length of chars faster than `Chars::count()`

![API](https://docs.rs/faster-chars-count/badge.svg)

Idea is from [UTF-8のコードポイントはどうやって高速に数えるか](https://qiita.com/saka1_p/items/ff49d981cfd56f3588cc), and [UTF-8のコードポイントはどうやってもっと高速に数えるか](https://qiita.com/umezawatakeshi/items/ed23935788756c800b86).

## usage
```
//before
"Hello, world!".chars().count();

//after
"Hello, world!".chars_count();
```

## bench
repeated "a" (only 1byte utf8)
![bench 1byte](lines_1.svg)

repeated "錆" (only 3byte utf8)
![bench 3byte](lines_3.svg)

See [performance bench](https://github.com/aobatact/faster_chars_count/tree/peformance) for details.

## future plan
sse (128bit)

avx512
[UTF-8のコードポイントはどうやってAVX-512で高速に数えるか](https://qiita.com/umezawatakeshi/items/fca066b2fd3dcf9cbec9)
