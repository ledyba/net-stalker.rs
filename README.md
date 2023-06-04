# net-stalker :: RSSフィーダ変換器

あるウェブサイトをスクレイプしてRSSに変換するウェブアプリです。

# 対応サイト

- [オープンセミナー | 東京大学ヒューマニティーズセンター（HMC）](https://hmc.u-tokyo.ac.jp/ja/open-seminar/)
- [植物Q&A | みんなのひろば | 日本植物生理学会](https://jspp.org/hiroba/q_and_a/)

# How to build and run

```bash
git clone git@github.com:ledyba/net-stalker.rs.git
cd net-stalker.rs
cargo build
./target/debug/net-stalker

# then, open http://lcoalhost:3000/kouan
```
