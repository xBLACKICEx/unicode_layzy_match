use deunicode::deunicode;

pub fn unicode_layzy_match<'a>(lst_uc: &Vec<&'a str>, usr_ipt: &str) -> Vec<&'a str> {
    if let Some(lst_uc_ascii) = unicde_first_ascii_char_mached(&lst_uc, usr_ipt) {
        if let Some(matched_lzy_uc) = uncode_layzy_pinyin_matched(&lst_uc_ascii, usr_ipt, &lst_uc) {
            return matched_lzy_uc;
        }

        if let Some(matched_uc) = unicode_ascii_matched(&lst_uc_ascii, usr_ipt, &lst_uc) {
            return matched_uc;
        }
    }
    vec![]
}

fn unicde_first_ascii_char_mached(lst_uc: &Vec<&str>, usr_ipt: &str) -> Option<Vec<(String, usize)>> {
    let mut lst_uni_ascii: Vec<(String, usize)> = Vec::new();

    lst_uc.iter().enumerate().for_each(|(i, hz)| {
        let tmp = deunicode(hz).to_lowercase();
        if tmp.starts_with(&usr_ipt[0..1]) {
            lst_uni_ascii.push((tmp, i));
        }
    });
    if !lst_uni_ascii.is_empty() {
       return Some(lst_uni_ascii);
    }
    None
}

fn uncode_layzy_pinyin_matched<'a>(lst_uni_ascii: &Vec<(String, usize)>, usr_ipt: &str, lst_uc: &Vec<&'a str>) -> Option<Vec<&'a str>> {
    let lazy_py: Vec<&(String, usize)> = lst_uni_ascii
        .iter()
        .filter(|(h, _)| match_layzy_pinyin(h.split_whitespace().collect(), usr_ipt))
        .collect();

    if !lazy_py.is_empty() {
        let l: Vec<_> = lazy_py.iter().map(|(_, i)| lst_uc[*i]).collect();
        return Some(l);
    }
    None
}

fn unicode_ascii_matched<'a>(lst_uc_ascii: &Vec<(String, usize)>, usr_ipt: &str, lst_uc: &Vec<&'a str>) -> Option<Vec<&'a str>> {
    let hz_py: Vec<&(String, usize)> = lst_uc_ascii
        .iter()
        .filter(|(h, _)| {
            let h = h
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .as_str()
                .to_lowercase();
            h.chars().zip(usr_ipt.chars()).all(|(s1, s2)| s1 == s2)
        })
        .collect();
    if !hz_py.is_empty() {
        let l: Vec<_> = hz_py.iter().map(|(_, i)| lst_uc[*i]).collect();
        return Some(l);
    }
    None
}

fn match_layzy_pinyin(lst_pinyin: Vec<&str>, pinyin: &str) -> bool {
    if lst_pinyin.len() != pinyin.len() { return false; }

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
        let lst_hz = vec!["不算", "北方", "不对", "よろしく", "こんにちわ", "同步", "彼方", "其他"];

        assert_eq!(vec!["よろしく"], unicode_layzy_match(&lst_hz, "y"));
        assert_eq!(vec!["こんにちわ"], unicode_layzy_match(&lst_hz, "kon"));
    }

    #[test]
    fn tets_unicode_layzy_match_witch_layzy_pytin_1() {
        let lst_hz = vec!["不算", "北方", "不对", "よろしく", "こんにちわ", "同步", "彼方", "其他"];

        assert_eq!(vec!["不算", "北方", "不对", "彼方"], unicode_layzy_match(&lst_hz, "b"));

        assert_eq!(vec!["北方", "彼方"], unicode_layzy_match(&lst_hz, "bf"));

        assert_eq!(vec!["北方"], unicode_layzy_match(&lst_hz, "beif"));
        assert_eq!(vec!["北方"], unicode_layzy_match(&lst_hz, "beifa"));
        assert_eq!(vec!["北方"], unicode_layzy_match(&lst_hz, "beifan"));
        assert_eq!(vec!["北方"], unicode_layzy_match(&lst_hz, "beifang"));
    }

    #[test]
    fn tets_unicode_layzy_match_witch_layzy_pytin_2() {
        let lst_hz = vec!["北方", "彼方", "其他", "不凡"];
        assert_eq!(vec!["北方", "彼方","不凡"], unicode_layzy_match(&lst_hz, "bf"));
        assert_eq!(vec!["不凡"], unicode_layzy_match(&lst_hz, "bfan")); // failed
    }
}
