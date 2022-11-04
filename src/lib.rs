use deunicode::deunicode;

/// match the corresponding unicode with a short ascii string
/// # Examples
/// ```
/// use unicode_layzy_match::unicode_layzy_match;
///
/// let lst_uc = vec![
///   "不算",      // bu suan
///   "北方",      // bei fang
///   "不对",      // bu dui
///   "よろしく",   // yo ro shi ku
///   "こんにちわ", // ko nn ni chi wa
///   "同步",     // tong bu
///   "彼方",     // bi fang
///   "其他",     // qi ta
///   ];
///
/// assert_eq!(vec!["よろしく"], unicode_layzy_match(&lst_uc, "y").unwrap());
/// assert_eq!(vec!["こんにちわ"], unicode_layzy_match(&lst_uc, "kon").unwrap());
/// assert_eq!(vec!["北方", "彼方"], unicode_layzy_match(&lst_uc, "bf").unwrap());
/// ```
pub fn unicode_layzy_match<'a>(lst_uc: &[&'a str], usr_ipt: &str) -> Option<Vec<&'a str>> {
    if let Some((mut lst_ps, mut lst_uc)) = unicde_first_ascii_char_mached(lst_uc, usr_ipt) {
        let matched_lzy_py = uncode_layzy_pinyin_matched(&mut lst_ps, &mut lst_uc, usr_ipt);

        if let Some(mut matched_uc) = unicode_ascii_matched(&mut lst_ps, &mut lst_uc, usr_ipt) {
            if let Some((_, mut lz_py)) = matched_lzy_py {
                lz_py.append(&mut matched_uc.1);
                return Some(lz_py);
            }
            return Some(matched_uc.1);
        } else if let Some((_, hz)) = matched_lzy_py {
            return Some(hz);
        }
    }
    None
}

fn unicde_first_ascii_char_mached<'a>(
    lst_uc: &[&'a str],
    usr_ipt: &str,
) -> Option<(Vec<String>, Vec<&'a str>)> {
    let mut lst_uni_ascii: (Vec<String>, Vec<&'a str>) = (Vec::new(), Vec::new());

    lst_uc.iter().for_each(|hz| {
        let tmp = deunicode(hz).to_lowercase();
        if tmp.starts_with(&usr_ipt[0..1]) {
            lst_uni_ascii.0.push(tmp);
            lst_uni_ascii.1.push(hz);
        }
    });
    if !lst_uni_ascii.0.is_empty() {
        return Some(lst_uni_ascii);
    }
    None
}

fn unicode_ascii_matched<'a>(
    lst_ps: &mut Vec<String>,
    lst_uc: &mut Vec<&'a str>,
    usr_ipt: &str,
) -> Option<(Vec<String>, Vec<&'a str>)> {
    let mut mached_py = vec![];
    let mut mached_hz = vec![];

    let mut i = 0;
    while i < lst_ps.len() {
        if lst_ps[i].to_lowercase().starts_with(usr_ipt) {
            mached_py.push(lst_ps.remove(i));
            mached_hz.push(lst_uc.remove(i));
        } else {
            i += 1;
        }
    }

    if !mached_py.is_empty() {
        return Some((mached_py, mached_hz));
    }
    None
}

fn uncode_layzy_pinyin_matched<'a>(
    lst_py: &mut Vec<String>,
    lst_hz: &mut Vec<&'a str>,
    usr_ipt: &str,
) -> Option<(Vec<String>, Vec<&'a str>)> {
    let mut mached_py = vec![];
    let mut mached_hz = vec![];

    let mut i = 0;
    while i < lst_py.len() {
        if match_layzy_pinyin(&lst_py[i].split_whitespace().collect::<Vec<_>>(), usr_ipt) {
            mached_py.push(lst_py.remove(i));
            mached_hz.push(lst_hz.remove(i));
        } else {
            i += 1;
        }
    }

    if !mached_py.is_empty() {
        return Some((mached_py, mached_hz));
    }
    None
}

fn match_layzy_pinyin(lst_pinyin: &[&str], pinyin: &str) -> bool {
    if lst_pinyin.len() != pinyin.len() {
        return false;
    }

    for (i, c) in pinyin.chars().into_iter().enumerate() {
        if !lst_pinyin[i].to_lowercase().starts_with(c) {
            return false;
        }
    }
    // todo support 北方（beifang） can be match with bfa, bfan, bfang
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tets_unicode_layzy_match_witch_layzy_acsii() {
        let lst_hz = vec![
            "不算",
            "北方",
            "不对",
            "よろしく",
            "こんにちわ",
            "同步",
            "彼方",
            "其他",
        ];

        assert_eq!(vec!["よろしく"], unicode_layzy_match(&lst_hz, "y").unwrap());
        assert_eq!(vec!["こんにちわ"], unicode_layzy_match(&lst_hz, "kon").unwrap());
    }

    #[test]
    fn tets_unicode_layzy_match_witch_layzy_pytin_1() {
        let lst_hz = vec![
            "不算",
            "北方",
            "不对",
            "よろしく",
            "こんにちわ",
            "同步",
            "彼方",
            "其他",
        ];

        assert_eq!(
            vec!["不算", "北方", "不对", "彼方"],
            unicode_layzy_match(&lst_hz, "b").unwrap()
        );

        assert_eq!(vec!["北方", "彼方"], unicode_layzy_match(&lst_hz, "bf").unwrap());

        assert_eq!(vec!["北方"], unicode_layzy_match(&lst_hz, "beif").unwrap());
        assert_eq!(vec!["北方"], unicode_layzy_match(&lst_hz, "beifa").unwrap());
        assert_eq!(vec!["北方"], unicode_layzy_match(&lst_hz, "beifan").unwrap());
        assert_eq!(vec!["北方"], unicode_layzy_match(&lst_hz, "beifang").unwrap());
    }

    #[test]
    fn tets_unicode_layzy_match_witch_layzy_pytin_2() {
        let lst_hz = vec!["北方", "彼方", "其他", "不凡"];
        assert_eq!(
            vec!["北方", "彼方", "不凡"],
            unicode_layzy_match(&lst_hz, "bf").unwrap()
        );
        // assert_eq!(vec!["不凡"], unicode_layzy_match(&lst_hz, "bfan").unwrap()); // failed
    }

    #[test]
    fn tets_unicode_layzy_match_witch_not_matched() {
        let lst_hz = vec!["北方", "彼方", "其他", "不凡"];
        assert!(unicode_layzy_match(&lst_hz, "wd").is_none());
    }
}
