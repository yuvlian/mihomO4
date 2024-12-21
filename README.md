# mihomO4

shitty mihomo api wrapper

## example usage:

add to deps:
```
mihomo4 = { git = "https://github.com/yuvlian/mihomo4" }
```

```rust
use mihomo4::{Mihomo, Language, Client};

#[tokio::main]
async fn main() {
    // we can reuse these since func parameter is
    // &Language and &Client
    let lang = Language::En;
    let client = Client::new();

    // get someone with uid 802775147 (me)
    let x = Mihomo::fetch_user(802775147, &lang, &client).await.unwrap();
    
    // self explanatory
    let player = x.get_nickname();
    let (lv, wlv) = x.get_lv_and_wlv();
    // clone cuz it returns &Ch and im lazy
    let ch = x.get_character_by_id("1107").unwrap().clone();
    let ch_name = ch.name.clone();
    let ch_lv = ch.level;
    let ch_max_lv = ch.max_level();
    let ch_lc = ch.light_cone.unwrap();
    let ch_relics = ch.relics;
    let ch_eidolon = ch.rank;

    println!("{}, lv:{}, wlv:{}\n", player, lv, wlv);
    println!("character: {}", ch_name);
    println!("lv: {}/{}, eidolon {}", ch_lv, ch_max_lv, ch_eidolon);
    println!("lc lv: {}/{}, superimp {}", ch_lc.level, ch_lc.max_level(), ch_lc.rank);
    println!("relics: {:#?}", ch_relics);
}
```