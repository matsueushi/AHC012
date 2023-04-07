# AHC012`

# AWSの認証情報
```shell
aws configure --profile dev
```
Default region name = ap-northeast-1

# Lambda関数の生成
```shell
cargo lambda new lambda-ahc-template
```
-> Cargo.tomlのmemberを追加

# Lambda関数のデプロイ
```shell
cargo lambda build --release --arm64
cargo lambda deploy --profile dev lambda-ahc-template
```

# 入力データを生成
```shell
cargo run --release --bin gen txt/seeds.txt
```