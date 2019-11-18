#![recursion_limit = "1024"]
#![feature(slice_concat_ext)]
extern crate wana_kana;

#[macro_use]
extern crate serde_json;
use serde_json::value::Value;

#[derive(Debug)]
struct Node {
    pub transitions: Option<Vec<(char, Node)>>,
    // pub output: &'static str
    pub output: String,
}

fn main() {
    let val = json!({
      "あ": {
        "": "a"
      },
      "い": {
        "": "i"
      },
      "う": {
        "": "u"
      },
      "え": {
        "": "e"
      },
      "お": {
        "": "o"
      },
      "か": {
        "": "ka"
      },
      "き": {
        "": "ki",
        "ゃ": {
          "": "kya"
        },
        "ゅ": {
          "": "kyu"
        },
        "ょ": {
          "": "kyo"
        },
        "ぃ": {
          "": "kyi"
        },
        "ぇ": {
          "": "kye"
        }
      },
      "く": {
        "": "ku",
        "ゃ": {
          "": "kya"
        },
        "ゅ": {
          "": "kyu"
        },
        "ょ": {
          "": "kyo"
        },
        "ぃ": {
          "": "kyi"
        },
        "ぇ": {
          "": "kye"
        }
      },
      "け": {
        "": "ke"
      },
      "こ": {
        "": "ko"
      },
      "さ": {
        "": "sa"
      },
      "し": {
        "": "shi",
        "ゃ": {
          "": "sha"
        },
        "ゅ": {
          "": "shu"
        },
        "ょ": {
          "": "sho"
        },
        "ぃ": {
          "": "shyi"
        },
        "ぇ": {
          "": "she"
        }
      },
      "す": {
        "": "su"
      },
      "せ": {
        "": "se"
      },
      "そ": {
        "": "so"
      },
      "た": {
        "": "ta"
      },
      "ち": {
        "": "chi",
        "ゃ": {
          "": "cha"
        },
        "ゅ": {
          "": "chu"
        },
        "ょ": {
          "": "cho"
        },
        "ぃ": {
          "": "chyi"
        },
        "ぇ": {
          "": "che"
        }
      },
      "つ": {
        "": "tsu"
      },
      "て": {
        "": "te"
      },
      "と": {
        "": "to"
      },
      "な": {
        "": "na"
      },
      "に": {
        "": "ni",
        "ゃ": {
          "": "nya"
        },
        "ゅ": {
          "": "nyu"
        },
        "ょ": {
          "": "nyo"
        },
        "ぃ": {
          "": "nyi"
        },
        "ぇ": {
          "": "nye"
        }
      },
      "ぬ": {
        "": "nu"
      },
      "ね": {
        "": "ne"
      },
      "の": {
        "": "no"
      },
      "は": {
        "": "ha"
      },
      "ひ": {
        "": "hi",
        "ゃ": {
          "": "hya"
        },
        "ゅ": {
          "": "hyu"
        },
        "ょ": {
          "": "hyo"
        },
        "ぃ": {
          "": "hyi"
        },
        "ぇ": {
          "": "hye"
        }
      },
      "ふ": {
        "": "fu",
        "ゃ": {
          "": "fya"
        },
        "ゅ": {
          "": "fyu"
        },
        "ょ": {
          "": "fyo"
        },
        "ぃ": {
          "": "fyi"
        },
        "ぇ": {
          "": "fye"
        }
      },
      "へ": {
        "": "he"
      },
      "ほ": {
        "": "ho"
      },
      "ま": {
        "": "ma"
      },
      "み": {
        "": "mi",
        "ゃ": {
          "": "mya"
        },
        "ゅ": {
          "": "myu"
        },
        "ょ": {
          "": "myo"
        },
        "ぃ": {
          "": "myi"
        },
        "ぇ": {
          "": "mye"
        }
      },
      "む": {
        "": "mu"
      },
      "め": {
        "": "me"
      },
      "も": {
        "": "mo"
      },
      "ら": {
        "": "ra"
      },
      "り": {
        "": "ri",
        "ゃ": {
          "": "rya"
        },
        "ゅ": {
          "": "ryu"
        },
        "ょ": {
          "": "ryo"
        },
        "ぃ": {
          "": "ryi"
        },
        "ぇ": {
          "": "rye"
        }
      },
      "る": {
        "": "ru"
      },
      "れ": {
        "": "re"
      },
      "ろ": {
        "": "ro"
      },
      "や": {
        "": "ya"
      },
      "ゆ": {
        "": "yu"
      },
      "よ": {
        "": "yo"
      },
      "わ": {
        "": "wa"
      },
      "ゐ": {
        "": "wi"
      },
      "ゑ": {
        "": "we"
      },
      "を": {
        "": "wo"
      },
      "ん": {
        "": "n",
        "あ": {
          "": "n'a"
        },
        "い": {
          "": "n'i"
        },
        "う": {
          "": "n'u"
        },
        "え": {
          "": "n'e"
        },
        "お": {
          "": "n'o"
        },
        "や": {
          "": "n'ya"
        },
        "ゆ": {
          "": "n'yu"
        },
        "よ": {
          "": "n'yo"
        }
      },
      "が": {
        "": "ga"
      },
      "ぎ": {
        "": "gi",
        "ゃ": {
          "": "gya"
        },
        "ゅ": {
          "": "gyu"
        },
        "ょ": {
          "": "gyo"
        },
        "ぃ": {
          "": "gyi"
        },
        "ぇ": {
          "": "gye"
        }
      },
      "ぐ": {
        "": "gu"
      },
      "げ": {
        "": "ge"
      },
      "ご": {
        "": "go"
      },
      "ざ": {
        "": "za"
      },
      "じ": {
        "": "ji",
        "ゃ": {
          "": "ja"
        },
        "ゅ": {
          "": "ju"
        },
        "ょ": {
          "": "jo"
        },
        "ぃ": {
          "": "jyi"
        },
        "ぇ": {
          "": "je"
        }
      },
      "ず": {
        "": "zu"
      },
      "ぜ": {
        "": "ze"
      },
      "ぞ": {
        "": "zo"
      },
      "だ": {
        "": "da"
      },
      "ぢ": {
        "": "ji",
        "ゃ": {
          "": "ja"
        },
        "ゅ": {
          "": "ju"
        },
        "ょ": {
          "": "jo"
        },
        "ぃ": {
          "": "jyi"
        },
        "ぇ": {
          "": "je"
        }
      },
      "づ": {
        "": "zu"
      },
      "で": {
        "": "de"
      },
      "ど": {
        "": "do"
      },
      "ば": {
        "": "ba"
      },
      "び": {
        "": "bi",
        "ゃ": {
          "": "bya"
        },
        "ゅ": {
          "": "byu"
        },
        "ょ": {
          "": "byo"
        },
        "ぃ": {
          "": "byi"
        },
        "ぇ": {
          "": "bye"
        }
      },
      "ぶ": {
        "": "bu"
      },
      "べ": {
        "": "be"
      },
      "ぼ": {
        "": "bo"
      },
      "ぱ": {
        "": "pa"
      },
      "ぴ": {
        "": "pi",
        "ゃ": {
          "": "pya"
        },
        "ゅ": {
          "": "pyu"
        },
        "ょ": {
          "": "pyo"
        },
        "ぃ": {
          "": "pyi"
        },
        "ぇ": {
          "": "pye"
        }
      },
      "ぷ": {
        "": "pu"
      },
      "ぺ": {
        "": "pe"
      },
      "ぽ": {
        "": "po"
      },
      "ゔぁ": {
        "": "va"
      },
      "ゔぃ": {
        "": "vi"
      },
      "ゔ": {
        "": "vu",
        "ゃ": {
          "": "vya"
        },
        "ゅ": {
          "": "vyu"
        },
        "ょ": {
          "": "vyo"
        },
        "ぃ": {
          "": "vyi"
        },
        "ぇ": {
          "": "vye"
        }
      },
      "ゔぇ": {
        "": "ve"
      },
      "ゔぉ": {
        "": "vo"
      },
      "。": {
        "": "."
      },
      "、": {
        "": ","
      },
      "：": {
        "": ":"
      },
      "・": {
        "": "/"
      },
      "！": {
        "": "!"
      },
      "？": {
        "": "?"
      },
      "〜": {
        "": "~"
      },
      "ー": {
        "": "-"
      },
      "「": {
        "": "‘"
      },
      "」": {
        "": "’"
      },
      "『": {
        "": "“"
      },
      "』": {
        "": "”"
      },
      "［": {
        "": "["
      },
      "］": {
        "": "]"
      },
      "（": {
        "": "("
      },
      "）": {
        "": ")"
      },
      "｛": {
        "": "{"
      },
      "｝": {
        "": "}"
      },
      "　": {
        "": " "
      },
      "ゃ": {
        "": "ya"
      },
      "ゅ": {
        "": "yu"
      },
      "ょ": {
        "": "yo"
      },
      "ぁ": {
        "": "a"
      },
      "ぃ": {
        "": "i"
      },
      "ぅ": {
        "": "u"
      },
      "ぇ": {
        "": "e"
      },
      "ぉ": {
        "": "o"
      },
      "っ": {
        "あ": {
          "": "a"
        },
        "い": {
          "": "i"
        },
        "う": {
          "": "u"
        },
        "え": {
          "": "e"
        },
        "お": {
          "": "o"
        },
        "か": {
          "": "kka"
        },
        "き": {
          "": "kki",
          "ゃ": {
            "": "kkya"
          },
          "ゅ": {
            "": "kkyu"
          },
          "ょ": {
            "": "kkyo"
          },
          "ぃ": {
            "": "kkyi"
          },
          "ぇ": {
            "": "kkye"
          }
        },
        "く": {
          "": "kku",
          "ゃ": {
            "": "kkya"
          },
          "ゅ": {
            "": "kkyu"
          },
          "ょ": {
            "": "kkyo"
          },
          "ぃ": {
            "": "kkyi"
          },
          "ぇ": {
            "": "kkye"
          }
        },
        "け": {
          "": "kke"
        },
        "こ": {
          "": "kko"
        },
        "さ": {
          "": "ssa"
        },
        "し": {
          "": "sshi",
          "ゃ": {
            "": "ssha"
          },
          "ゅ": {
            "": "sshu"
          },
          "ょ": {
            "": "ssho"
          },
          "ぃ": {
            "": "sshyi"
          },
          "ぇ": {
            "": "sshe"
          }
        },
        "す": {
          "": "ssu"
        },
        "せ": {
          "": "sse"
        },
        "そ": {
          "": "sso"
        },
        "た": {
          "": "tta"
        },
        "ち": {
          "": "tchi",
          "ゃ": {
            "": "tcha"
          },
          "ゅ": {
            "": "tchu"
          },
          "ょ": {
            "": "tcho"
          },
          "ぃ": {
            "": "tchyi"
          },
          "ぇ": {
            "": "tche"
          }
        },
        "つ": {
          "": "ttsu"
        },
        "て": {
          "": "tte"
        },
        "と": {
          "": "tto"
        },
        "な": {
          "": "na"
        },
        "に": {
          "": "ni",
          "ゃ": {
            "": "nya"
          },
          "ゅ": {
            "": "nyu"
          },
          "ょ": {
            "": "nyo"
          },
          "ぃ": {
            "": "nyi"
          },
          "ぇ": {
            "": "nye"
          }
        },
        "ぬ": {
          "": "nu"
        },
        "ね": {
          "": "ne"
        },
        "の": {
          "": "no"
        },
        "は": {
          "": "hha"
        },
        "ひ": {
          "": "hhi",
          "ゃ": {
            "": "hhya"
          },
          "ゅ": {
            "": "hhyu"
          },
          "ょ": {
            "": "hhyo"
          },
          "ぃ": {
            "": "hhyi"
          },
          "ぇ": {
            "": "hhye"
          }
        },
        "ふ": {
          "": "ffu",
          "ゃ": {
            "": "ffya"
          },
          "ゅ": {
            "": "ffyu"
          },
          "ょ": {
            "": "ffyo"
          },
          "ぃ": {
            "": "ffyi"
          },
          "ぇ": {
            "": "ffye"
          }
        },
        "へ": {
          "": "hhe"
        },
        "ほ": {
          "": "hho"
        },
        "ま": {
          "": "mma"
        },
        "み": {
          "": "mmi",
          "ゃ": {
            "": "mmya"
          },
          "ゅ": {
            "": "mmyu"
          },
          "ょ": {
            "": "mmyo"
          },
          "ぃ": {
            "": "mmyi"
          },
          "ぇ": {
            "": "mmye"
          }
        },
        "む": {
          "": "mmu"
        },
        "め": {
          "": "mme"
        },
        "も": {
          "": "mmo"
        },
        "ら": {
          "": "rra"
        },
        "り": {
          "": "rri",
          "ゃ": {
            "": "rrya"
          },
          "ゅ": {
            "": "rryu"
          },
          "ょ": {
            "": "rryo"
          },
          "ぃ": {
            "": "rryi"
          },
          "ぇ": {
            "": "rrye"
          }
        },
        "る": {
          "": "rru"
        },
        "れ": {
          "": "rre"
        },
        "ろ": {
          "": "rro"
        },
        "や": {
          "": "ya"
        },
        "ゆ": {
          "": "yu"
        },
        "よ": {
          "": "yo"
        },
        "わ": {
          "": "wwa"
        },
        "ゐ": {
          "": "wwi"
        },
        "ゑ": {
          "": "wwe"
        },
        "を": {
          "": "wwo"
        },
        "ん": {
          "": "n"
        },
        "が": {
          "": "gga"
        },
        "ぎ": {
          "": "ggi",
          "ゃ": {
            "": "ggya"
          },
          "ゅ": {
            "": "ggyu"
          },
          "ょ": {
            "": "ggyo"
          },
          "ぃ": {
            "": "ggyi"
          },
          "ぇ": {
            "": "ggye"
          }
        },
        "ぐ": {
          "": "ggu"
        },
        "げ": {
          "": "gge"
        },
        "ご": {
          "": "ggo"
        },
        "ざ": {
          "": "zza"
        },
        "じ": {
          "": "jji",
          "ゃ": {
            "": "jja"
          },
          "ゅ": {
            "": "jju"
          },
          "ょ": {
            "": "jjo"
          },
          "ぃ": {
            "": "jjyi"
          },
          "ぇ": {
            "": "jje"
          }
        },
        "ず": {
          "": "zzu"
        },
        "ぜ": {
          "": "zze"
        },
        "ぞ": {
          "": "zzo"
        },
        "だ": {
          "": "dda"
        },
        "ぢ": {
          "": "jji",
          "ゃ": {
            "": "jja"
          },
          "ゅ": {
            "": "jju"
          },
          "ょ": {
            "": "jjo"
          },
          "ぃ": {
            "": "jjyi"
          },
          "ぇ": {
            "": "jje"
          }
        },
        "づ": {
          "": "zzu"
        },
        "で": {
          "": "dde"
        },
        "ど": {
          "": "ddo"
        },
        "ば": {
          "": "bba"
        },
        "び": {
          "": "bbi",
          "ゃ": {
            "": "bbya"
          },
          "ゅ": {
            "": "bbyu"
          },
          "ょ": {
            "": "bbyo"
          },
          "ぃ": {
            "": "bbyi"
          },
          "ぇ": {
            "": "bbye"
          }
        },
        "ぶ": {
          "": "bbu"
        },
        "べ": {
          "": "bbe"
        },
        "ぼ": {
          "": "bbo"
        },
        "ぱ": {
          "": "ppa"
        },
        "ぴ": {
          "": "ppi",
          "ゃ": {
            "": "ppya"
          },
          "ゅ": {
            "": "ppyu"
          },
          "ょ": {
            "": "ppyo"
          },
          "ぃ": {
            "": "ppyi"
          },
          "ぇ": {
            "": "ppye"
          }
        },
        "ぷ": {
          "": "ppu"
        },
        "ぺ": {
          "": "ppe"
        },
        "ぽ": {
          "": "ppo"
        },
        "ゔぁ": {
          "": "vva"
        },
        "ゔぃ": {
          "": "vvi"
        },
        "ゔ": {
          "": "vvu",
          "ゃ": {
            "": "vvya"
          },
          "ゅ": {
            "": "vvyu"
          },
          "ょ": {
            "": "vvyo"
          },
          "ぃ": {
            "": "vvyi"
          },
          "ぇ": {
            "": "vvye"
          }
        },
        "ゔぇ": {
          "": "vve"
        },
        "ゔぉ": {
          "": "vvo"
        },
        "。": {
          "": "."
        },
        "、": {
          "": ","
        },
        "：": {
          "": ":"
        },
        "・": {
          "": "/"
        },
        "！": {
          "": "!"
        },
        "？": {
          "": "?"
        },
        "〜": {
          "": "~"
        },
        "ー": {
          "": "-"
        },
        "「": {
          "": "‘"
        },
        "」": {
          "": "’"
        },
        "『": {
          "": "“"
        },
        "』": {
          "": "”"
        },
        "［": {
          "": "["
        },
        "］": {
          "": "]"
        },
        "（": {
          "": "("
        },
        "）": {
          "": ")"
        },
        "｛": {
          "": "{"
        },
        "｝": {
          "": "}"
        },
        "　": {
          "": " "
        },
        "ゃ": {
          "": "ya"
        },
        "ゅ": {
          "": "yu"
        },
        "ょ": {
          "": "yo"
        },
        "ぁ": {
          "": "a"
        },
        "ぃ": {
          "": "i"
        },
        "ぅ": {
          "": "u"
        },
        "ぇ": {
          "": "e"
        },
        "ぉ": {
          "": "o"
        },
        "": ""
      }
    });

    get_transitions(&val);

    println!("{:#?}", get_transitions(&val));
    // println!("{:?}", serde_json::to_string_pretty(&get_transitions(&val)));
}

fn get_transitions(val: &Value) -> Option<Vec<(char, Node)>> {
    let mut transitions = vec![];

    if let Some(obj) = val.as_object() {
        for (k, v) in obj.iter() {
            if k == "" {
                continue;
            }

            // if v.as_object().unwrap().get("").is_none(){
            //   print!("{:?}", k,);
            // }

            let node = Node {
                transitions: get_transitions(&v),
                output: v.as_object().unwrap().get("").unwrap().as_str().unwrap().to_string(),
            };
            let char = k.chars().next().unwrap();
            transitions.push((char, node));
        }
    } else {
        panic!("waaaa");
    }

    if transitions.is_empty() {
        None
    } else {
        Some(transitions)
    }
}
