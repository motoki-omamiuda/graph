# graph

## 考えたい問題
- [ ] 新しい道建設の優先度評価  
    コスト/需要/効果から、新たに建設すべき道（重み付きの辺）を評価する方法を考える  
    流れ：
    1. [Laporte (2000)](https://idus.us.es/server/api/core/bitstreams/07065e7c-ee1c-4400-a348-af45b4b931eb/content)で「鉄道のルート設計の条件」の全体像を掴む
    2. 交通工学の教科書で「利用者均衡配分（どうやって人の流れをシミュレートするか）」の数式を理解する
    3.  [Holliday](https://cim.mcgill.ca/~mrl/pubs/ahollid/ITSC_2023.pdf)の論文で「GNN（グラフニューラルネットワーク）」への落とし込み方を理解

## 実装するもの
- [ ] グラフの埋め込み（[1] p49）

## 参考
[1] 佐藤竜馬, *グラフニューラルネットワーク*  
[2] Robin J. Wilson, *Introduction to Graph Theory*
