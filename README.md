# latex2chin ⚡ Rust 驱动的 LaTeX 转中文

> 基于 Rust 构建的高性能 LaTeX 数学表达式 → 中文读法转换工具。

`latex2chin` 核心解析引擎使用 Rust 编写（通过 [PyO3](https://pyo3.rs) + [Maturin](https://www.maturin.rs) 提供 Python 绑定），解析速度极快，同时结合 LLM Agent 实现对混合中文文本中 LaTeX 片段的自动识别与转换。

---

## 安装（从源码构建）

### 前置要求

- Python >= 3.9
- [Rust 工具链](https://rustup.rs/)
- 推荐使用 [uv](https://docs.astral.sh/uv/) 管理 Python 环境

### 使用 uv（推荐）

```bash
git clone https://github.com/cocolato/latex2chin.git
cd latex2chin
uv sync
source .venv/bin/activate
```

### 使用 pip

```bash
git clone https://github.com/cocolato/latex2chin.git
cd latex2chin

python -m venv .venv
source .venv/bin/activate
pip install maturin
maturin develop
pip install -e .
```

## 快速开始

```python
from latex2chin import parse_latex

print(parse_latex("\\frac{1}{2} + 3"))
# 输出: 2分之1加3
```

---

## API 参考

### `parse_latex(latex: str) -> str`

将**单个** LaTeX 数学表达式直接转换为中文文本。  
该函数由 Rust 实现，执行速度极快，不依赖网络。

**参数：**

| 参数 | 类型 | 说明 |
|------|------|------|
| `latex` | `str` | LaTeX 数学表达式字符串 |

**返回值：** `str` — 对应的中文读法

**示例：**

```python
from latex2chin import parse_latex

parse_latex("1 + 2")           # "1加2"
parse_latex("-3.14")           # "负3.14"
parse_latex("\\frac{1}{2}")    # "2分之1"
parse_latex("\\sqrt{2}")       # "2的平方根"
parse_latex("\\sqrt[3]{8}")    # "8的立方根"
parse_latex("50%")             # "百分之50"
parse_latex("100\\degree")     # "100度"
parse_latex("\\pm2")           # "正负2"
parse_latex("\\pi \\approx 3.14")  # "派约等于3.14"
parse_latex("2 \\times 3")    # "2乘3"
parse_latex("6 \\div 2")      # "6除以2"
parse_latex("1 \\neq 2")      # "1不等于2"
parse_latex("1 \\leq 2")      # "1小于等于2"
```

---

### `parse_chinese(string: str, *, api_key=None, base_url=None, model=None) -> str`

利用 LLM Agent 自动识别一段**混合中文文本**中的 LaTeX / 数学表达式，将其转换为中文后返回完整文本。  
适用于将教学讲义、题目解析等含有公式的段落一键转为纯中文表述。

> 该函数需要联网调用 LLM API，兼容 OpenAI 格式请求（默认使用 DeepSeek 的 OpenAI 兼容接口）。

**参数：**

| 参数 | 类型 | 说明 |
|------|------|------|
| `string` | `str` | 包含 LaTeX 表达式的中文文本 |
| `api_key` | `str \| None` | LLM API Key，可选；未传则使用全局配置 |
| `base_url` | `str \| None` | OpenAI 格式 API 地址，可选；未传则使用全局配置 |
| `model` | `str \| None` | 模型名称，可选；未传则使用全局配置 |

**返回值：** `str` — 公式已被转换为中文的完整文本

**示例：**

```python
from latex2chin import parse_chinese, configure

# 方式一：通过 configure 全局设置
configure(
    api_key="sk-xxx",
    base_url="https://api.deepseek.com",
    model="deepseek-chat",
)

result = parse_chinese(
    "让我们来看下面这道题,题目给出式子:\\frac{1}{2}+3*4,"
    "为了处理这个式子,我们可以先计算3*4"
)
print(result)
# 让我们来看下面这道题,题目给出式子:2分之1加3乘4,为了处理这个式子,我们可以先计算3乘4

# 方式二：调用时直接传入
result = parse_chinese(
    "求解 \\sqrt{2} + \\pi",
    api_key="sk-xxx",
    base_url="https://api.deepseek.com",
    model="deepseek-chat",
)
# 求解 2的平方根加派
```

---

### `configure(*, api_key=None, base_url=None, model=None)`

全局配置 LLM 连接参数，供 `parse_chinese` 使用。  
也可以通过环境变量设置。

**参数：**

| 参数 | 类型 | 默认值 / 环境变量 |
|------|------|-------------------|
| `api_key` | `str \| None` | 环境变量 `LATEX2CHIN_API_KEY` |
| `base_url` | `str \| None` | 环境变量 `LATEX2CHIN_BASE_URL`（默认 `https://api.deepseek.com`） |
| `model` | `str \| None` | 环境变量 `LATEX2CHIN_MODEL`（默认 `deepseek-chat`） |

**示例：**

```python
from latex2chin import configure

# 代码中配置
configure(api_key="sk-xxx", model="deepseek-chat")

# 或使用环境变量（shell 中设置）
# export LATEX2CHIN_API_KEY=sk-xxx
# export LATEX2CHIN_BASE_URL=https://api.deepseek.com
# export LATEX2CHIN_MODEL=deepseek-chat
```

---

### `settings`

全局配置对象（`_Settings` 实例），可直接读取当前配置状态。

```python
from latex2chin import settings

print(settings.api_key)
print(settings.base_url)
print(settings.model)
```

---

## 支持的 LaTeX 语法

### 基础语法

| 类别 | 语法示例 | 输出 |
|------|---------|---------|
| 整数/小数 | `42`, `3.14` | `42`, `3.14` |
| 变量 | `x`, `A`, `n` | `x`, `A`, `n` |
| 正负号 | `+5`, `-3` | `正5`, `负3` |
| 加法 | `1 + 2` | `1加2` |
| 减法 | `5 - 3` | `5减3` |
| 乘法 | `2 * 3`, `2 \times 3`, `2 \cdot 3` | `2乘3` |
| 除法 | `6 / 2`, `6 \div 2` | `6除以2` |
| 分数 | `\frac{1}{2}`, `\dfrac{1}{2}`, `\tfrac{1}{2}` | `2分之1` |
| 根号 | `\sqrt{2}` | `2的平方根` |
| 立方根 | `\sqrt[3]{8}` | `8的立方根` |
| N次方根 | `\sqrt[n]{x}` | `x的n次方根` |
| 正负 | `\pm 2`, `±2` | `正负2` |
| 负正 | `\mp 2` | `负正2` |
| 百分号 | `50%` | `百分之50` |
| 度数 | `100\degree`, `100°` | `100度` |
| 括号 | `(1 + 2)` | `1加2`（括号省略）|

### 上标与下标

| 类别 | 语法示例 | 输出 |
|------|---------|---------|
| 平方 | `x^2` | `x的平方` |
| 立方 | `x^3` | `x的立方` |
| N次方 | `x^n`, `x^{n+1}` | `x的n次方`, `x的n加1次方` |
| 下标 | `a_1`, `a_{n+1}` | `a1`, `an加1` |

### 比较运算符

| 类别 | 语法示例 | 输出 |
|------|---------|---------|
| 等于 | `=` | `等于` |
| 不等于 | `\neq`, `\ne` | `不等于` |
| 小于 | `<`, `\lt` | `小于` |
| 大于 | `>`, `\gt` | `大于` |
| 小于等于 | `\leq`, `\le` | `小于等于` |
| 大于等于 | `\geq`, `\ge` | `大于等于` |
| 约等于 | `\approx`, `≈` | `约等于` |
| 不约等于 | `\not\approx`, `≉` | `不约等于` |

### 希腊字母

| 语法 | 输出 |
|------|------|
| `\alpha` | `阿尔法` |
| `\beta` | `贝塔` |
| `\gamma` | `伽马` |
| `\delta` | `德尔塔` |
| `\epsilon` | `艾普西龙` |
| `\theta` | `西塔` |
| `\lambda` | `兰姆达` |
| `\mu` | `缪` |
| `\sigma` | `西格玛` |
| `\pi` | `派` |
| `\phi` | `fai` |
| `\omega` | `欧米伽` |

### 三角函数

| 语法 | 输出 |
|------|------|
| `\sin x` | `sinx` |
| `\cos x` | `cosx` |
| `\tan x` | `tanx` |
| `\cot x` | `cotx` |
| `\sec x` | `secx` |
| `\csc x` | `cscx` |

### 对数函数

| 语法 | 输出 |
|------|------|
| `\log x` | `logx` |
| `\ln x` | `lnx` |
| `\lg x` | `以10为底的对数x` |

### 微积分

| 类别 | 语法示例 | 输出 |
|------|---------|---------|
| 极限 | `\lim_{x \to 0} x` | `x趋近于0时x的极限` |
| 求和 | `\sum_{i=1}^{n} i` | `对i从1到n的i求和` |
| 求积 | `\prod_{i=1}^{n} i` | `对i从1到n求积` |
| 积分 | `\int_{a}^{b} x` | `从a到b的x的定积分` |

### 集合论

| 语法 | 输出 |
|------|------|
| `x \in A` | `x属于A` |
| `x \notin A` | `x不属于A` |
| `A \cup B` | `A并B` |
| `A \cap B` | `A交B` |
| `A \subset B` | `A是B的子集` |
| `A \supset B` | `A是B的超集` |
| `\emptyset` | `空集` |

### 逻辑符号

| 语法 | 输出 |
|------|------|
| `P \forall x` | `P任意x` |
| `P \exists x` | `P存在x` |
| `A \Rightarrow B`, `A \implies B` | `A推出B` |
| `A \iff B`, `A \Leftrightarrow B` | `A等价于B` |

### 几何符号

| 语法 | 输出（二元运算） | 输出（单独使用） |
|------|-----------------|-------------|
| `\triangle` | — | `三角形` |
| `\angle` | — | `角` |
| `A \parallel B` | `A平行于B` | — |
| `A \perp B` | `A垂直于B` | — |
| `A \cong B` | `A全等于B` | — |
| `A \sim B` | `A相似于B` | — |

---

## 许可证

MIT
