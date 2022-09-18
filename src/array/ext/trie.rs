//! # 字典树
//! 
//! * [616. 给字符串添加加粗标签](add_bold_tag)
//! * [648. 单词替换](replace_words)

enum State {
    End,
    Inner,
    NotWord,
}

// 字典树暴露的接口就两个
// 一个是添加单词, 一个是查询是否包含
trait TrieTree {
    fn insert(&mut self, s: &str);
    fn find(&self, s: &str) -> State;
}

use std::collections::HashMap;

/// 存储节点
/// 自身没有一个value字段存储自身是哪个字符
/// 是由上层父节点的hashmap里的key决定的
/// 对应的有一个头节点, 这样代码处理会统一一些
pub struct TrieNode {
    is_final: bool,                       // 标示到这里, 是不是一个有效单词
    child_nodes: HashMap<char, TrieNode>, // 可以用数组覆盖ascii码, hashmap也行
}

impl TrieNode {
    fn create(_: char, is_final: bool) -> Self {
        Self {
            // value: Some(value),
            is_final,
            child_nodes: HashMap::new(),
        }
    }
    fn create_root() -> Self {
        Self {
            is_final: false,
            child_nodes: HashMap::new(),
        }
    }
}

struct TrieStruct {
    root: TrieNode, //第一层用起来, 但不加虚节点,会导致第一层需要特殊处理.
}

impl TrieStruct {
    fn create() -> Self {
        Self {
            root: TrieNode::create_root(),
        }
    }
}

impl TrieTree for TrieStruct {
    fn insert(&mut self, s: &str) {
        let mut current_node = &mut self.root;

        for chr in s.chars() {
            current_node = current_node
                .child_nodes
                .entry(chr)
                .or_insert_with(|| TrieNode::create(chr, false));
        }
        current_node.is_final = true;
    }
    fn find(&self, s: &str) -> State {
        let mut current_node = &self.root;

        for chr in s.chars() {
            if let Some(node) = current_node.child_nodes.get(&chr) {
                current_node = node;
                continue;
            }
            return State::NotWord;
        }
        if current_node.is_final {
            return State::End;
        }
        State::Inner
    }
}

/// [616. 给字符串添加加粗标签](https://leetcode.cn/problems/add-bold-tag-in-string/)
///
/// 每次全遍历words,找前缀太过低效
/// 可以将words中的单词按照首字母存map, 然后索引
/// 但如果首字母相同, 第二个字母又会出现上面的遍历.
/// 因此可以使用字典树
pub fn add_bold_tag(s: String, words: Vec<String>) -> String {
    let mut trie_tree = TrieStruct::create();
    for word in words.iter() {
        trie_tree.insert(word.as_str());
    }

    let mut mark: Vec<(usize, usize)> = vec![];
    for i in 0..s.len() {
        let mut curr = &trie_tree.root; // 保存当前起点下, 对应的层级.
        for j in i..s.len() {
            let chr = s.chars().nth(j).unwrap();
            match curr.child_nodes.get(&chr) {
                Some(nxt) => {
                    curr = nxt; // 切换到下一层

                    if !curr.is_final {
                        continue;
                    }
                    if let Some(last) = mark.last_mut() {
                        if i >= last.0 && i <= last.1 {
                            last.0 = last.0.min(i);
                            last.1 = last.1.max(j + 1);
                        } else {
                            mark.push((i, j + 1));
                        }
                    } else {
                        mark.push((i, j + 1));
                    }
                }
                None => {
                    break;
                }
            }
        }
    }

    let mut ans = String::new();
    let mut curr = 0;
    for m in mark {
        ans.push_str(s.get(curr..m.0).unwrap());
        ans.push_str("<b>");
        ans.push_str(s.get(m.0..m.1).unwrap());
        ans.push_str("</b>");
        curr = m.1;
    }
    if curr < s.len() {
        ans.push_str(s.get(curr..).unwrap());
    }

    ans
}

/// [648. 单词替换](https://leetcode.cn/problems/replace-words/)
pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
    let mut trie_tree = TrieStruct::create();
    for word in dictionary.iter() {
        trie_tree.insert(word.as_str());
    }

    let mut result_tmp = vec![];
    'NextWord: for part in sentence.split_ascii_whitespace() {
        let mut curr = &trie_tree.root; // 保存当前起点下, 对应的层级.
        for (i, chr) in part.chars().enumerate() {
            match curr.child_nodes.get(&chr) {
                Some(nxt) => {
                    curr = nxt; // 切换到下一层
                    if curr.is_final {
                        // 取最短的
                        result_tmp.push(part.get(..i + 1).unwrap().to_string());
                        continue 'NextWord;
                    }
                }
                None => {
                    result_tmp.push(part.to_string());
                    continue 'NextWord;
                }
            }
        }
        result_tmp.push(part.to_string());
    }
    result_tmp.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_words() {
        struct TestCase {
            name: &'static str,
            dictionary: &'static [&'static str],
            sentence: &'static str,
            expect: &'static str,
        }

        vec![
            TestCase {
                name: "basic 1",
                dictionary: &["cat", "bat", "rat"],
                sentence: "the cattle was rattled by the battery",
                expect: "the cat was rat by the bat",
            },
            TestCase {
                name: "basic 2",
                dictionary: &["a", "b", "c"],
                sentence: "aadsfasf absbs bbab cadsfafs",
                expect: "a a b c",
            },
            TestCase{
                name:"fix 1",
                dictionary: &["e","k","c","harqp","h","gsafc","vn","lqp","soy","mr","x","iitgm","sb","oo","spj","gwmly","iu","z","f","ha","vds","v","vpx","fir","t","xo","apifm","tlznm","kkv","nxyud","j","qp","omn","zoxp","mutu","i","nxth","dwuer","sadl","pv","w","mding","mubem","xsmwc","vl","farov","twfmq","ljhmr","q","bbzs","kd","kwc","a","buq","sm","yi","nypa","xwz","si","amqx","iy","eb","qvgt","twy","rf","dc","utt","mxjfu","hm","trz","lzh","lref","qbx","fmemr","gil","go","qggh","uud","trnhf","gels","dfdq","qzkx","qxw"],
                sentence: "ikkbp miszkays wqjferqoxjwvbieyk gvcfldkiavww vhokchxz dvypwyb bxahfzcfanteibiltins ueebf lqhflvwxksi dco kddxmckhvqifbuzkhstp wc ytzzlm gximjuhzfdjuamhsu gdkbmhpnvy ifvifheoxqlbosfww mengfdydekwttkhbzenk wjhmmyltmeufqvcpcxg hthcuovils ldipovluo aiprogn nusquzpmnogtjkklfhta klxvvlvyh nxzgnrveghc mpppfhzjkbucv cqcft uwmahhqradjtf iaaasabqqzmbcig zcpvpyypsmodtoiif qjuiqtfhzcpnmtk yzfragcextvx ivnvgkaqs iplazv jurtsyh gzixfeugj rnukjgtjpim hscyhgoru aledyrmzwhsz xbahcwfwm hzd ygelddphxnbh rvjxtlqfnlmwdoezh zawfkko iwhkcddxgpqtdrjrcv bbfj mhs nenrqfkbf spfpazr wrkjiwyf cw dtd cqibzmuuhukwylrnld dtaxhddidfwqs bgnnoxgyynol hg dijhrrpnwjlju muzzrrsypzgwvblf zbugltrnyzbg hktdviastoireyiqf qvufxgcixvhrjqtna ipfzhuvgo daee r nlipyfszvxlwqw yoq dewpgtcrzausqwhh qzsaobsghgm ichlpsjlsrwzhbyfhm ksenb bqprarpgnyemzwifqzz oai pnqottd nygesjtlpala qmxixtooxtbrzyorn gyvukjpc s mxhlkdaycskj uvwmerplaibeknltuvd ocnn frotscysdyclrc ckcttaceuuxzcghw pxbd oklwhcppuziixpvihihp",
                expect: "i miszkays w gvcfldkiavww v dvypwyb bxahfzcfanteibiltins ueebf lqhflvwxksi dc k w ytzzlm gximjuhzfdjuamhsu gdkbmhpnvy i mengfdydekwttkhbzenk w h ldipovluo a nusquzpmnogtjkklfhta k nxzgnrveghc mpppfhzjkbucv c uwmahhqradjtf i z q yzfragcextvx i i j gzixfeugj rnukjgtjpim h a x h ygelddphxnbh rvjxtlqfnlmwdoezh z i bbfj mhs nenrqfkbf spfpazr w c dtd c dtaxhddidfwqs bgnnoxgyynol h dijhrrpnwjlju muzzrrsypzgwvblf z h q i daee r nlipyfszvxlwqw yoq dewpgtcrzausqwhh q i k bqprarpgnyemzwifqzz oai pnqottd nygesjtlpala q gyvukjpc s mxhlkdaycskj uvwmerplaibeknltuvd ocnn f c pxbd oklwhcppuziixpvihihp"
            }
        ]
        .iter()
        .for_each(|testcase| {
            let dictionary = testcase.dictionary.iter().map(|s| s.to_string()).collect();
            let actual = replace_words(dictionary, testcase.sentence.to_string());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_add_bold_tag() {
        struct TestCase {
            name: &'static str,
            s: &'static str,
            words: &'static [&'static str],
            expect: &'static str,
        }

        vec![
                TestCase {
                    name: "basic",
                    s: "abcxyz123",
                    words: &["abc", "123"],
                    expect: "<b>abc</b>xyz<b>123</b>",
                },
                TestCase {
                    name: "basic 2",
                    s: "aaabbcc",
                    words: &["aaa", "aab", "bc"],
                    expect: "<b>aaabbc</b>c",
                },
                TestCase {
                    name: "fix 1",
                    s: "aaabbcc",
                    words: &[],
                    expect: "aaabbcc",
                },
                TestCase{
                    name: "fix 2",
                    s: "xhhjzbkvpmasiypsqqjobufcqmlhdjffrdohsxgksftaekzhwzydhbfdiylihnvjlvpoptnqigszckimljbepgisnmyszfsxkxyfdfqngytfuihepohapvhbyhqydvroflfnsyjmygtykdejfudrhxxawcewgiguiwsvqrgbxrbdnrvguzjftqcsjbvjlbxfsvzpdpmtlzobwnxrtgisbcqmhugncjwgatfctydryakvbnmlbiftndfefylsmlebzdumefuflwhtwijtrhhhmknklalgqjaoicmnywtvzldbeftkydjsdkkonayhdxhrjazosqloilagcwzeezavnsqelxqhtlzymedxmkrovxhkrgfenyhxgdroeejedbwpnkqbqknalwgxoxweyxngorvrpnfkvagdqkbtuayaihyhwcsdtjzzvxfavrhzgf",
                    words: &["xh","hj","zb","kv","pm","as","iy","ps","qq","jo","bu","fc","qm","lh","dj","ff","rd","oh","sx","gk","sf","ta","ek","zh","wz","yd","hb","fd","li","hn","vj","lv","po","pt","nq","ig","sz","ck","im","lj","be","pg","is","nm","ys","zf","kx"],
                    expect: "<b>xhhjzbkvpmasiypsqqjobufcqmlhdjffrdohsxgksftaekzhwzydhbfdiylihnvjlvpoptnqigszckimljbepgisnmyszfsxkx</b>y<b>fd</b>fqngytfuihe<b>poh</b>apv<b>hb</b>yhq<b>yd</b>vroflfnsyjmygtykdejfudrhxxawcewg<b>ig</b>uiwsvqrgbxrbdnrvguzjftqcsjb<b>vj</b>lbxfsvzpd<b>pm</b>tlzobwnxrtg<b>is</b>bc<b>qm</b>hugncjwgat<b>fc</b>t<b>yd</b>rya<b>kv</b>b<b>nm</b>lbiftndfefylsmlebzdumefuflwhtwijtrhhhmknklalgqjaoicmnywtvzld<b>be</b>ftk<b>ydj</b>sdkkonayhd<b>xh</b>rjazosqloilagc<b>wz</b>eezavnsqelxqhtlzymedxmkrov<b>xh</b>krgfenyhxgdroeejedbwpnkqbqknalwgxoxweyxngorvrpnf<b>kv</b>agdqkbtuayaihyhwcsdtjzzvxfavrhzgf"
                }
            ]
            .iter()
            .for_each(|testcase| {
                let words = testcase.words.iter().map(|s| s.to_string()).collect();
                let actual = add_bold_tag(testcase.s.to_string(), words);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
    }
}
