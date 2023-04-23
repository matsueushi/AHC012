# AHC012

#  AtCoder 10th Anniversary
問題 -> https://atcoder.jp/contests/ahc012/tasks/ahc012_a
ビジュアライザ -> https://img.atcoder.jp/ahc012/f756367b32.html?lang=ja

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
cargo run --bin solver < in/0000.txt > out/local/0000.txt
```

# Lambda関数の実行
[LambdaにS3の読み込み権限を付与する](https://dev.classmethod.jp/articles/get-s3-object-with-python-in-lambda/)必要あり
```shell
aws --profile dev lambda invoke --function-name lambda-ahc012 --payload '{ "bucket_in" : "procon-inputs", "bucket_out" : "procon-outputs", "contest_name": "ahc012", "seed": 0 }' out/cloud/test.json
```

# テスト
```shell
cargo test -- --nocapture
```

# 参考
https://github.com/rust-lang-ja/atcoder-rust-resources
https://github.com/rust-lang-ja/atcoder-rust-base/tree/ja

全てコピー->ggVGy
