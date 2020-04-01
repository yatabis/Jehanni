### phase:1 〜整数変数定義〜 (最新版)

#### このフェーズでできるようになること

整数リテラルを用いた整数変数の定義

#### 文字

| 意味     | 名称       | 定義                    |
| -------- | ---------- | ----------------------- |
| 英小文字 | small      | `"a" | "b" | ... | "z"` |
| 英大文字 | large      | `"A" | "B" | ... | "Z"` |
| 文字     | letter     | `small | large | "_"`   |
| 数字     | digit      | `"0" | "1" | ... | "9"` |

#### 要素

| 意味         | 名称        | 定義                        |
| ------------ | ----------- | --------------------------- |
| 識別子       | identifier  | `letter { letter | digit }` |
| 整数リテラル | int_literal | `{ digit }`                 |
| 定義記号 | definion | `":="`                  |
| 改行記号 | newline    | `"\n"`                     |

#### 構造

ソースコードは **変数定義** (var_definition) の羅列 (<NEWLINE> 区切り)

**変数定義** は **変数名** (var_name) に **値** (value) を紐付ける

**変数名** は **識別子** (identifier)

**値** は **整数リテラル** (int_literal)

| 意味         | 名称           | 定義                      |
| ------------ | -------------- | ------------------------- |
| 変数名       | var_name       | `identifier`              |
| 値           | value          | `int_literal`             |
| 変数定義     | var_definition | `var_name definion value` |
| 行           | line           | `var_definition newline`  |
| ソースコード | source         | `{ line }`                |

#### 例

```phase_1.jh
x := 3
y := 200

test01 := 01
```



### phase:2 〜数の拡張（数・実数・整数・小数）〜 (開発中)

#### このフェーズでできるようになること

（暗黙的な）数の型という概念

小数リテラルを用いた浮動小数点型変数の定義

#### 文字

| 意味     | 名称   | 定義                    |
| -------- | ------ | ----------------------- |
| 英小文字 | small  | `"a" | "b" | ... | "z"` |
| 英大文字 | large  | `"A" | "B" | ... | "Z"` |
| 文字     | letter | `small | large | "_" `  |
| 数字     | digit  | `"0" | "1" | ... | "9"` |

#### 要素

| 意味         | 名称                                 | 定義                                                  |
| ------------ | ------------------------------------ | ----------------------------------------------------- |
| 識別子       | identifier                           | `letter | { letter | digit }`                         |
| 整数リテラル | int_literal                          | `{ digit }`                                           |
| 小数リテラル | <font color=red>float_literal</font> | <font color=red>`int_literal "."  int_literal`</font> |
| 実数リテラル | <font color=red>real_literal</font>  | <font color=red>`int_literal | float_literal`</font>  |
| 数リテラル   | <font color=red>num_literal</font>   | <font color=red>`real_literal`</font>                 |
| 定義記号     | definion                             | `":="`                                                |
| 改行記号     | newline                              | `"\n"`                                                |

#### 構造

ソースコードは **変数定義** (var_definition) の羅列 (<NEWLINE> 区切り)

**変数定義** は **変数名** (var_name) に **値** (value) を紐付ける

**変数名** は **識別子** (identifier)

**値** は **数リテラル** (num_literal)

**数リテラル** は **実数リテラル** (real_literal)

**実数リテラル** は **整数リテラル** (int_literal) または **小数リテラル** (float_literal)

| 意味         | 名称           | 定義                          |
| ------------ | -------------- | ----------------------------- |
| 変数名       | var_name       | `identifier`                  |
| 値           | value          | <font color=red>`num_literal`</font> |
| 変数定義     | var_definition | `var_name definion value`     |
| 行           | line           | `var_definition newline`      |
| ソースコード | source         | `{ line }`                    |

#### 例

```
x := 3
y := 0.5
z := 12.345
a10 := 10.0
```

