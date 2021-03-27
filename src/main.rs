// https://nlp100.github.io/ja/
fn main() {
    p002();
}

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
    let v = vec![1, 2, 3, 4, 5];

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

    for (count, word) in vw.into_iter() {
        println!("{}", word);
    }
}

pub fn p004() {

}