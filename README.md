# AHC012

# AWSの認証情報
```shell
aws configure --profile dev
```
Default region name = ap-northeast-1

# Lambda関数の生成
```shell
cargo lambda new lambda-ahc012
```
-> Cargo.tomlのmemberを追加

# Lambda関数のデプロイ
```shell
cargo lambda build --release --arm64
cargo lambda deploy --profile dev lambda-ahc012
```

# 入力データを生成
```shell
cargo run --release --bin gen txt/seeds.txt
```

# 入力データをS3に保存
```shell
aws --profile dev s3 cp ./in s3://procon-inputs/ahc012 --recursive
```

# ローカルでの実行
```shell
cargo run --bin solver < in/0000.txt
```

# 参考
https://github.com/rust-lang-ja/atcoder-rust-resources
https://github.com/rust-lang-ja/atcoder-rust-base/tree/ja
