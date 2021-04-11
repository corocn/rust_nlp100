// 文字列”stressed”の文字を逆に（末尾から先頭に向かって）並べた文字列を得よ
pub fn p000() {
    let str = "stressed";
    let chars = str.chars();
    let rev_str: String = chars.rev().collect();
    println!("{}", rev_str);
}

// 「パタトクカシーー」という文字列の1,3,5,7文字目を取り出して連結した文字列を得よ．
pub fn p001() {
    let str = String::from("パタトクカシー");
    let str: String = str
        .chars()
        .enumerate()
        .filter_map(|(index, char)| if index % 2 == 0 { Some(char) } else { None })
        .collect();
    println!("{}", str);
}

// 「パトカー」＋「タクシー」の文字を先頭から交互に連結して文字列「パタトクカシーー」を得よ．
pub fn p002() {
    let str1 = String::from("パトカー");
    let str2 = String::from("タクシー");

    let chars1: Vec<_> = str1.chars().collect();
    let chars2: Vec<_> = str2.chars().collect();

    let mut chars = vec![];

    for (index, char) in chars1.into_iter().enumerate() {
        // dbg!(char1);
        let x1 = char;
        let x2 = *chars2.get(index).unwrap();

        chars.push(x1.to_string());
        chars.push(x2.to_string());
    }

    // dbg!(&chars);

    let joined: String = chars.join("");

    println!("{}", joined);
}

// 単語に分解し，各単語の（アルファベットの）文字数を先頭から出現順に並べたリストを作成せよ．
pub fn p003() {
    let _v = vec![1, 2, 3, 4, 5];

    let str = String::from("Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.");
    let split_word = &[' ', ',', '.'][..];
    let v: Vec<_> = str.split(split_word).filter(|x| *x != "").collect();

    let mut vw = vec![];

    for word in v.into_iter() {
        let chars = word.chars();
        let len = chars.count();

        let word_with_count = (len, word);
        vw.push(word_with_count);
    }

    vw.sort_by(|a, b| b.0.cmp(&a.0));

    for (_count, word) in vw.into_iter() {
        println!("{}", word);
    }
}

// 単語に分解し，1, 5, 6, 7, 8, 9, 15, 16, 19番目の単語は先頭の1文字，それ以外の単語は先頭の2文字を取り出し，
// 取り出した文字列から単語の位置（先頭から何番目の単語か）への連想配列（辞書型もしくはマップ型）を作成せよ．

use std::collections::HashMap;

pub fn p004() {
    let text = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let text = text.replace(".", "");

    let words: Vec<_> = text.split(" ").collect();

    let first_letters = vec![1, 5, 6, 7, 8, 9, 15, 16, 19];

    let mut element2word = HashMap::new();

    for (index, word) in words.into_iter().enumerate() {
        let n = index + 1;

        let pick = if first_letters.contains(&n) {
            let first = word.chars().next().unwrap().to_string();
            first
        } else {
            let mut p = word.chars();
            let first = p.next().unwrap().to_string();
            let second = p.next().unwrap().to_string();

            format!("{}{}", &first, &second)
        };

        element2word.insert(pick, n);
    }

    dbg!(&element2word.get("Be").unwrap());
}

// 与えられたシーケンス（文字列やリストなど）からn-gramを作る関数を作成せよ．
// この関数を用い，”I am an NLPer”という文から単語bi-gram，文字bi-gramを得よ．
pub fn p005() {
    let text = "I am an NLPer";
    let list: Vec<&str> = text.split(" ").collect();
    let v = create_bigram(&list, 2);
    dbg!(v);

    let text = "I am an NLPer";
    let list: Vec<&str> = text.split("").filter(|&x| x != "").collect();
    let v = create_bigram(&list, 2);
    dbg!(v);
}

pub fn create_bigram<'a>(list: &'a Vec<&str>, n: usize) -> Vec<Vec<&'a str>> {
    let mut bigram: Vec<Vec<&str>> = vec![];

    let len = list.len();

    for i in (0..len) {
        let mut add = vec![];

        for x in (0..n) {
            match list.get(i + x) {
                Some(x) => add.push(*x),
                None => ()
            }
        }

        if add.len() == n {
            bigram.push(add);
        }
    }

    bigram
}

// “paraparaparadise”と”paragraph”に含まれる文字bi-gramの集合を，
// それぞれ, XとYとして求め，XとYの和集合，積集合，差集合を求めよ．さらに，’se’というbi-gramがXおよびYに含まれるかどうかを調べよ．
pub fn p006() {
    let text1 = String::from("paraparaparadise");
    // dbg!(text1);
}
