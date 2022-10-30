# unicode_layzy_match

The purpose of this project is to match the corresponding unicode with a short ascii string, instead of switching to the input method of the corresponding language.

## Currently supports:
 - Pinyin inert (vowel initials) matching.
 - Compact Unicode matching with other languages (only Japanese hiragana and katakana have been tested for now).

## Known bugs:
 - All 漢字（ｊｐ　Kanji）(ｚｈ　Hanzi) will be matched into pinyin. \
  like　Japanese kanji 桜（sakura）will be recognized as Chinese pinyin（樱/櫻）ying

## Todo Next
 - improve pinyin completion
 - Support double spelling(pinyin)
 - Support Wu Bi
 - Realize ZH,JP Kanji Ascii correspondence distinction.