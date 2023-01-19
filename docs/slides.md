# Project: ZF

A love letter to the command line.

https://github.com/yue4u/zf

![](/logo.png)

---
layout: center
---

# What

What's the outcome? どんなゲームなの？

This is an game about using command line to accomplish tasks and fight in space.

このゲームはコマンドラインを操作しながらタスクを完了し、敵を倒していくゲームである。

---

# Commands

Commands implemented 実装したコマンド

All `209` different commands available.

ゲーム内で計 `209` それぞれ違うコマンドが利用できる。

そのうち
- `39` self implemented / 自前で実装した
- `170` from `nushell` / `nushell`由来

Example:

`game`, ..., `level`, ..., `tutorial`, `hint`, `engine`, ..., `shield`, ... , `repair`, `task`, ... , `ui`, `audio`, `term`, `radar`, `fire`, `time`, ... , `special thanks`, `mystery`, `credits` ...

---

# Categories

分類

They can be put into 3 categories.

三種類に分類できる

- `game`
- `system`
- `game & system`

---
layout: two-cols
---

# System

システム

Type `game start` to start to game

- `game start` を入力してゲームを始める
- `game end` で終了
- その他の部分も
  - `level`
  - `audio`
  - `term`
  - `...`

::right::

![](/start.avif)

---
layout: two-cols
---

`level next` で次のレベルに行く

![](/level.avif)

::right::

`term opcaticy .1` で透過度を操作

![](/opacity.avif)

---
layout: center
---

システム自体でもコマンドで操作できることで、

ゲームコンセプトの統一感を保つ

---
layout: two-cols
---

# Game

ゲーム


Game related commands, directly affect game play.

一番ゲームプレイと直結するコマンド群.

- `engine`
- `fire`
- `radar`
- `task`
- `...`

::right::

`engine thruter 100` で移動する

![](/engine.avif)

---
layout: two-cols
---

`fire beam` で攻撃

![](/fire.avif)

::right::

`radar` で敵の位置を取得

![](/radar.avif)

---
layout: center
---

一つ一つのギミック、コマンド操作前提の面白さを意識してで設計した

---

# Game & System

- `time scale`
  - 特殊の立ち位置
  - システムに対して操作することでゲームプレイに影響する

![](/time.avif)

---

# Game play

実際のゲームプレイ内容として

## 飛行制御

![](/control.avif)

---

## 武器システムのタイミング

![](/weapon.avif)

---

## ビジュアル自体の楽しさ

...などを中心内容にした

![](/screenshots/2.jpg)

---
layout: center
---

これらだけではなく、

プログラミングのチュートリアルとしてデザインした一面もある

---

# Advanced topics

アドバンストの内容

ゲームレベルデザインは`nushell`のチュートリアルを参考しながら制作した。

ゲームの進行と共に、以下の少し複雑のテーマが出てくて、

既知なコマンドをうまく組み合わせてクリアするのがポイントとなる

- data pipeline / filtering
- background task
- events tigger

---

# data pipeline / filtering

- dataを処理して次に渡す
- 例として
  - `radar | get 0 | fire hm`
  - `radar | where type != hazard`

![](/filter.png)

---

# background task / events tigger

- 一定時間で自動実行
  - `task run 'fire beam'`
  - `task run 'radar | get 0 | e rel'`

- イベントが起きたときの自動処理
  - `task on radiation_area_entered 'shield on; time scale 5'`
---
layout: center
---

# Player support

プレイヤーサポート

ゲーム自体が複雑な内容を扱うため、操作性やプレイヤーを適度に助ける機能も実装した

---

## 入力の補完

![](completion.png)

---

## エラーメッセージ (powerd by miette)

![](/error.png)

---

# How

This game is made with

- [Godot Engine](https://godotengine.org/) - game engine
- [Rust](https://www.rust-lang.org/) - the best programming language
- [GDNative](https://github.com/godot-rust/gdnative) - api bindings
- [WASI (The WebAssembly System Interface)](https://wasi.dev/) & [Wasmtime](https://wasmtime.dev/) - execution sandbox
- [nushell](https://www.nushell.sh/) - command parsing / execution
- [wezterm](https://wezfurlong.org/wezterm/) - in-game terminal emulator

---

# Architecture

アーキテクチャ

ゲーム内で`入力` -> `構文解析` -> `実行` -> `レンダリング` の流れ全部実装した

![](/architecture.png)

---

# Runtime

実行環境

- 入力: player -> term -> runtime -> shell
- 出力: shell -> host funciton -> runtime
  1. runtime -> term
  2. host function -> engine -> shell -> runtime -> term

---

WASIの処理化

```rust
let engine = Engine::default();
let mut linker = Linker::<ExtendedStore<S>>::new(&engine);
wasmtime_wasi::add_to_linker(&mut linker, |s| &mut s.wasi)?;

...

let wasi = WasiCtxBuilder::new()
    .stdout(Box::new(stdout.clone()))
    .stderr(Box::new(stderr.clone()))
    .build();
let mut store = Store::new(
    ...
);

let zf_shell_module = Module::from_binary(&engine, SHELL_WASM)?;
prepare(&mut linker)?;

let instance = linker.instantiate(&mut store, &zf_shell_module)?;
linker.instance(&mut store, SHELL_MODULE, instance)?;
```

---

host関数定義

```rust
#[cfg(target_family = "wasm")]
#[link(wasm_import_module = "zf")]
extern "C" {
    /// args: tag of ZFCommandArgs
    /// return: tag of String
    fn zf_cmd(args: i64) -> i64;
    /// return: combined width & height
    fn zf_terminal_size() -> i64;
}
```


host関数注入

```rust
Runtime::init(self, |linker| -> anyhow::Result<()> {
    linker
        .func_wrap("zf", "zf_cmd", RuntimeFunc::zf_cmd)?;
        .func_wrap("zf", "zf_terminal_size", RuntimeFunc::zf_terminal_size)?
    Ok(())
})
```

---

# Rendering
レンダリング

godotのAPIを利用してansiのレンダリング対応

-> godot-xtermを参考しながら必要な機能だけ実装した

## 行数計算

```rust
fn calc_terminal_size(base: TRef<Control>, cell_size: Vector2) -> TerminalSize {
    let rect = base.get_rect();
    let rows = ((rect.size.y - TERM_PADDING * 2.) / cell_size.y).floor() as usize;
    let cols = ((rect.size.x - TERM_PADDING * 2.) / cell_size.x).floor() as usize;

    TerminalSize {
        rows,
        cols,
        ..Default::default()
    }
}
```

---

## 文字レンダリング


```rust
fn draw_pos(&self, x: f32, y: f32) -> Vector2 {
    Vector2 {
        x: TERM_PADDING + x * self.base_cell_size.x,
        // position uses bottom-left so 2x here
        y: 2. * TERM_PADDING + y * self.base_cell_size.y,
    }
}

...

base.draw_string(
    &self.font,
    self.draw_pos(x, y),
    cell.str(),
    Color::from_rgba(fg.0, fg.1, fg.2, fg.3),
    -1,
);
```

詳細: https://github.com/yue4u/zf にて公開中

---

# Shader

本ゲームの制作においてshaderを多用して様々なビジュアルエフェクトを作り出した

タイトルのグリッチからはじめ、

![](title.avif)

---
layout: two-cols
---

その他のマティリアルなどでも使用した


![](/weapon.avif)

::right::

<br/>

<br/>

<br/>

![](material.avif)

---

シールドのShader、 edgeのラインを描画する部分

```glsl
void fragment() {
  ...

	vec4 c = tex(VERTEX.xy + TIME);
	ALBEDO = hsv2rgb(c.xyz + 0.01 * VERTEX);
	ALPHA = min(max(0.,fract(c.x * 2.) - (.95 - hit)) * 2., .05);

  float gap = 0.25;
  vec2 distance = gap / 2. - abs(gap / 2. - mod(UV.xy, gap));
  float distance_from_edge = min(
      distance.x,
      distance.y
  );
  if(distance_from_edge < 0.001) {
      ALPHA = 1.;
  }
}
```
---

その中、HPバーが一番良く作れたと感じる

![](hp.avif)

---
layout: two-cols
---

# Logo

ロゴだけblenderを利用した

![](/logo.png)

::right::

![](/blender.png)


---

# その他のアセット

- フォント
  - JetBrains Mono
  - Google Font
- 画像
  - フリー素材
- モデル
  - sketchfab
- BGM/SE
  - https://dova-s.jp
- Libraries
  - https://crates.io


---
layout: center
---

# Why:

Why this game/topic? このテーマを選んだ理由

I live in the command line and always fascinated by the power of it.

私はコマンドラインラインで生活していて、いつもそれの力に魅了されている。

---

# Thanks

Ad astra abyssosque!

ご清聴ありがとうございました。

![](/clear.avif)