# js_ergo — Roadmap / TODO

定位:**JS 风格、零依赖、Unicode 安全的字符串扩展层**。
只补 std 给不了的（B 桶），不重复 std（A 桶），不拖重依赖（C 桶）。

贯穿全局的两条铁律：
- **负索引一律用 `isize` 参数**（`usize` 收不了负数）。
- **长度/索引一律按 `char`（Unicode 标量），不是 JS 的 UTF-16。** 文档顶部统一声明一次；
  emoji / 代理对上与 JS 行为不同是「特性」，不是 bug，要写清楚。

---

## ✅ 已完成

- [x] `pad_start(length, pad)` — char / &str / &String / String，多字符 pattern 重复截断
- [x] `pad_end(length, pad)`
- [x] `PadWith` sealed trait 基础设施（新增 pad 类型要在 `mod sealed` + `impl PadWith` 两处加）
- [x] 发布 0.1.0 / 0.2.0，打 tag v0.1.0 / v0.2.0

---

## 🎯 0.3.0 — 立住「安全字符串索引」身份（旗舰）

### [ ] `slice(start, end) -> &str`  ★★★★★ 痛点 / ★★★☆ 难度
- 签名:`fn slice(&self, start: isize, end: isize) -> &str`（零拷贝借用，别返回 String）
- JS 语义:负数从尾部数 / 越界自动 clamp / `start >= end` 返回 `""`
- 实现核心:用 `char_indices()` 把「字符索引（含负）」映射到**字节偏移**，
  结果永远落在字符边界上（这正是 std 不替你做、JS 人最痛的点）
- 边界测试:负索引、越界、start==end、start>end、空串、emoji 不被切碎、纯 ASCII 快路径

### [ ] `slice_from(start) -> &str`  （配套,解决 JS `slice(2)` 省略 end）
- `fn slice_from(&self, start: isize) -> &str`，等价 `slice(start, len)`
- 或者评估用 range 风格 API 统一处理可选 end —— **动手前定死 API 形状**

### [ ] `at(index) -> Option<char>`  ★★★★☆ 痛点 / ★☆☆☆ 难度（快赢）
- `fn at(&self, index: isize) -> Option<char>`
- 负索引 = `len + index`;越界返回 `None`（比 JS 的 `undefined` 更地道）
- 覆盖了非负的 `charAt`,所以 **`char_at` 不单独做**

> 发布 0.3.0 后,库的故事就从「补丁函数」变成「JS 风格的安全索引/切片」。

---

## 0.4.0 — 字符位置查找

### [ ] `index_of(pat) -> Option<usize>`  ★★★☆☆ / ★★☆☆
- 返回**字符**下标(不是 std `find` 的字节下标),用 `Option` 替代 JS 的 `-1` 哨兵
- 实现:`find` 拿字节位 → `self[..byte].chars().count()` 换算字符位

### [ ] `last_index_of(pat) -> Option<usize>`  同上,基于 `rfind`

---

## 待定 / 选做（价值有限,看反馈再说）

### [ ] `code_point_at(index) -> Option<u32>`  ★★☆☆☆ / ★☆☆☆
- 几乎零成本(`at(i).map(|c| c as u32)`),但需求 niche

### [ ] `substring(start, end)`  ★★☆☆☆ / ★★☆☆
- 仅为 JS 怪癖兼容(无负数、start>end 自动交换参数)
- 有 `slice` 后基本冗余 —— 只有「想逐行移植 JS」才做

---

## ❌ 明确不做（避免「全家桶」广度陷阱）

- **A 桶(std 已有,做了贬值)**:`startsWith` `endsWith` `includes` `repeat`
  `split` `trim*` `toLowerCase/Upper` `concat` `replace` `replaceAll`
- **C 桶(拖重依赖,毁掉零依赖卖点)**:`match`/`matchAll`/`search`(regex)、
  `normalize`(unicode-normalization)、`localeCompare`/`toLocale*`(ICU)

---

## 每个新方法的完成标准（对齐 pad_* 的质量条）

- [ ] 文档:`# Examples` + 必要时 `# Panics`,以及 char vs UTF-16 的差异说明
- [ ] 测试:基础 / no-op / 越界 / 负索引 / 空串 / 多字节不被切碎 / 长度不变量
- [ ] `cargo test`(含 doctest)+ `cargo clippy --all-targets` 全绿
- [ ] 新 pad 类型才需要动 `sealed` + `PadWith`;索引类方法不涉及

---

## 营销 / 发现（真想要用户时再做）

- [ ] README 放「vs `format!` / vs std」对比表,主动正面回应「为什么不用 std」
- [ ] r/rust 发帖 + 投 This Week in Rust
- [ ] 参考既有竞品 `voca_rs` / `Inflector` 的定位与采用度,设定合理预期(中等偏低)
