Library for counting length of chars faster than `Chars::count()`

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
[bench 1byte](peformance/lines_1.svg)

repeated "錆" (only 3byte utf8)
[bench 1byte](peformance/lines_3.svg)

## future plan
sse (128bit)

avx512
[UTF-8のコードポイントはどうやってAVX-512で高速に数えるか](https://qiita.com/umezawatakeshi/items/fca066b2fd3dcf9cbec9)
