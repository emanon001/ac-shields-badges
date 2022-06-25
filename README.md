# ac-shields-badges

[Shields](https://shields.io/)で AtCoder のバッジを作成するための JSON endpoint。

![](https://img.shields.io/endpoint?url=https%3A%2F%2Fac-shields-badges.vercel.app%2Fapi%2Fac-rate%3Fuser_id%3Demanon001%26contest_type%3Dalgorithm)
![](https://img.shields.io/endpoint?url=https%3A%2F%2Fac-shields-badges.vercel.app%2Fapi%2Fac-rate%3Fuser_id%3Demanon001%26contest_type%3Dheuristic)

バッジの URL は以下の JavaScript コードで取得できます。

```js
const getBadgeUrl = (userId, contestType) => {
  const endpoint = encodeURIComponent(
    `https://ac-shields-badges.vercel.app/api/ac-rate?user_id=${userId}&contest_type=${contestType}`
  );
  return `https://img.shields.io/endpoint?url=${endpoint}`;
};
const userId = "emanon001"; // AtCoderユーザーID
console.table({
  algorithm: getBadgeUrl(userId, "algorithm"),
  heuristic: getBadgeUrl(userId, "heuristic"),
});
```
