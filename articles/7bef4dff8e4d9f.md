---
title: "Rust and gRPC | Getting Started"
emoji: "🦁"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: []
published: false
---
Rust で gRPC を実装する方法を調べる。

現職では、Backend API は、ほとんどが Go。

Go は、書き方を制限されているため、beginner が誤った書き方をすることによる問題が発生しにくくてとても良い。
例えば、Java のような複雑な仕様はない。

しかし、Go 一択というのも問題があると思うので、他の選択肢として Rust を学ぶ。

Rust は、memory safe、性能といった点で、他の言語よりも優れている。
Edge でも動かすことができるくらい 軽量にできる。例えば、Cloudflare Workers など。

## environment
- Devcontainer
   - `Cargo` などの setup が不要。
   - VS Code Extension も default。

## Getting Started
公式の docs を手元で試してみるのが一番分かりやすい。
僕の場合は、Rust で写経して、Go で rewrite してというのを繰り返した。
基本的な syntax は、繰り返し書くことで覚える。

[Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

## gRPC
`Connect-Go` で実装した code を Rust で rewrite する。
proto は、超単純なこれ。同じものを使う。

https://github.com/danny-yamamoto/go-connect-greet/blob/main/weather/v1/weather.proto

## generate proto
`build.rs` で generate する。
`build.rs` は、build 時に最初に実行される code。root 直下に置く。
初見では、難しそうに見えるが、1つ1つ見ていくとそうでもない。

https://github.com/danny-yamamoto/rust-grpc/blob/main/Weather/build.rs

GPT-4 の要約。
`weather_descriptor.bin` は、meta 情報を取得するため作成。
>  このコードは、Rustのプログラムで、gRPCのサーバを構築するための設定とコンパイルを行っています。
tonic_buildクレートを使用して、gRPCのサーバ機能を有効にし、ファイル記述子セットのパスを設定し、そして特定のプロトコルバッファファイル（weather.proto）をコンパイルしています。
コードはまた、環境変数 OUT_DIR を使用して出力ディレクトリのパスを取得し、そこに weather_descriptor.bin という名前でファイル記述子セットを保存します。

## HTTP Server

https://github.com/danny-yamamoto/rust-grpc/blob/main/Weather/src/main.rs

GPT-4 の要約。
> このRustプログラムは、gRPCのサーバーを作成し、単純な天気サービスを提供します。次の主な点が含まれています：
> 1. モジュールと依存関係のインポート:
std::net::SocketAddr, tonic, および tonic_reflection などのモジュールと依存関係をインポートします。
> 1. Protocol Buffers定義のインポート:
mod weather {tonic::include_proto!("weather.v1");} で定義されたProtocol Buffersのスキーマをインポートします。
> 1. 天気サービスの実装:
StructWeatherService という名前の構造体を作成し、WeatherService トレイトを実装します。この実装には、weather メソッドが含まれ、WeatherRequest を受け取り、WeatherResponse を返します。
> 1. 非同期メイン関数の定義:
#[tokio::main] アトリビュートを使用して非同期のメイン関数を定義し、gRPCサーバーを起動します。このサーバーは、作成した天気サービスを含み、指定されたアドレスでリッスンします。
> 1. gRPCサーバーの構築と起動:
tonic::transport::Server::builder() を使用してgRPCサーバーを構築し、作成した天気サービスを追加してサーバーを起動します。
> 
> このプログラムは、指定されたアドレスでgRPCサーバーを起動し、クライアントからの天気関連のリクエストに対応するシンプルな天気サービスを提供する例です。

`grpcurl` で操作を確認。
```bash
grpcurl -plaintext -d '{"condition": "Cloudy"}' localhost:8080 weather.v1.WeatherService/Weather
{
  "text": "Today's weather is Cloudy!"
}
```
