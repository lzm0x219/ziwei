# JavaScript 双后端最小语言绑定

## 决策

JavaScript 语言绑定拆为两个锁步版本包：

- `@ziweijs/core`：Node.js NAPI，提供 ESM 与 CommonJS 入口。
- `@ziweijs/core-wasm`：浏览器原生 ESM，显式异步初始化 WebAssembly 后使用同步领域 interface。

两个包的首个纵向切片只公开：

```ts
type Gender = "Yin" | "Yang";

interface ZiweiBirth {
  readonly gender: Gender;
  readonly year: number;
  readonly month: number;
  readonly day: number;
  readonly hour: number;
}

class Ziwei {
  private constructor();
  static fromBirth(birth: ZiweiBirth): Ziwei;
}
```

`ZiweiBirth` 是 TypeScript 纯类型和 JavaScript plain object，不是运行时 class。字段名、`Gender` 值、月日时起点与 Rust `ZiweiBirth` 完全一致。两个 Adapter 都执行表示范围检查，然后调用 `ZiweiBirth::try_new` 与 `Ziwei::from_birth`；不复制排盘规则。

`Ziwei` 是不透明、不可直接构造的只读对象。NAPI 与 WASM 内部句柄由各运行时 GC 管理，不公开 `dispose` 或 `free`。首版不提供查询方法，因此它只验证包加载、双后端一致性、输入错误和对象生命周期。

## 初始化与同步语义

Node 包无需初始化：

```ts
import { Ziwei } from "@ziweijs/core";
const chart = Ziwei.fromBirth(birth);
```

WASM 包只允许一条显式初始化路径；初始化完成后 `fromBirth` 同步：

```ts
import init, { Ziwei } from "@ziweijs/core-wasm";
await init(wasmUrl);
const chart = Ziwei.fromBirth(birth);
```

默认初始化由生成的 ESM 相对定位 `.wasm`，也接受调用方提供的 URL、`Response`、buffer 或 `WebAssembly.Module`。

## Interface 与 Adapter

公共 TypeScript 领域声明只有一份源文件，构建时生成到两个包。napi-rs 与 wasm-bindgen 生成的声明仅供实现核对，不作为公共 interface。

| Module                        | Implementation                          |
| ----------------------------- | --------------------------------------- |
| `ziwei`                       | 领域模型、输入不变量和排盘管线          |
| `ziwei_binding` native target | NAPI Adapter 与 Node 原生句柄           |
| `ziwei_binding` wasm target   | wasm-bindgen Adapter 与浏览器句柄       |
| 两个 npm 包                   | 加载器、公共 facade、共享声明和契约测试 |

预期输入失败统一暴露为 `ZiweiInputError`，`code` 为 `INVALID_INPUT`。底层 NAPI、wasm-bindgen 和反序列化错误不直接进入公共 interface。

## 延后

- `Ziwei.fromInput()` 与所有查询方法。
- `toJSON()`、静态快照和 JSON schema。
- 中文标签、locale 与显示元数据。
- Node WASM、Deno、Bun、Edge、Worker 自动化和多线程 WASM。
- 平台原生包矩阵和发布 workflow。

不保留旧 `createChart`、`@matharts/ziwei` 或 re-export 占位路径。
