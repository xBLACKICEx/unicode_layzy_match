# unicode_layzy_match

The purpose of this project is to match the corresponding unicode with a short ascii string, instead of switching to the input method of the corresponding language.
![图片](https://user-images.githubusercontent.com/51036094/198887439-039e5e6b-669b-44d0-9685-fec94c4d2fe9.png)

## Currently supports:
 - Pinyin inert (vowel initials) matching.
 - Compact Unicode matching with other languages (only Japanese hiragana and katakana have been tested for now).

## Known bugs:
 - All 漢字（ｊｐ　Kanji）(ｚｈ　Hanzi) will be matched into pinyin. \
  like　Japanese kanji 桜（sakura）will be recognized as Chinese pinyin（樱/櫻）ying

## Todo Next
 - improve pinyin completion
 - Support double spelling(双拼)
 - Support Wu Bi
 - Realize ZH,JP Kanji Ascii correspondence distinction.
