# net-stalker :: RSSフィーダ変換器

一部のウェブサイトをスクレイプしてRSSに変換するウェブアプリです。

# 対応サイト

- [オープンセミナー | 東京大学ヒューマニティーズセンター（HMC）](https://hmc.u-tokyo.ac.jp/ja/open-seminar/)
- [植物Q&A | みんなのひろば | 日本植物生理学会](https://jspp.org/hiroba/q_and_a/)
- [感染症発生動向調査週報 (IDWR)](https://www.niid.go.jp/niid/ja/idwr.html)
- [産総研：取得した資産の需要調査](https://www.aist.go.jp/aist_j/procure/asset/jyuyou.html)
- [プレスリリース | JAMSTEC | 海洋研究開発機構](https://www.jamstec.go.jp/j/about/press_release/)

# How to build and run

```bash
git clone git@github.com:ledyba/net-stalker.rs.git
cd net-stalker.rs
cargo build
./target/debug/net-stalker
```

Then, open [http://lcoalhost:3000/hmc](http://lcoalhost:3000/hmc), for example.
