# ac-shields-badges

[Shields](https://shields.io/)で AtCoder のバッジを作成するための JSON endpoint。

![](https://img.shields.io/endpoint?url=https%3A%2F%2Fac-shields-badges.vercel.app%2Fapi%2Fac-rate%3Fuser_id%3Demanon001%26contest_type%3Dalgorithm)
![](https://img.shields.io/endpoint?url=https%3A%2F%2Fac-shields-badges.vercel.app%2Fapi%2Fac-rate%3Fuser_id%3Demanon001%26contest_type%3Dheuristic)

バッジの URL は以下の通りです。

`https://ac-shields-badges.vercel.app/api/ac-rate?user_id=${user_id}&contest_type=${contest_type}`

| パラメータ     | 説明                           |
| -------------- | ------------------------------ |
| `user_id`      | AtCoder のユーザー ID          |
| `contest_type` | `algorithm` または `heuristic` |
