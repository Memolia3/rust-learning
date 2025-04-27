# 最新のRustイメージを使用
FROM rust:1.81.0-slim

# 作業ディレクトリを設定
WORKDIR /usr/src/app

# タイムゾーンを設定
ENV TZ=Asia/Tokyo

# 必要なパッケージをインストール
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# cargo-watchをインストール
RUN cargo install --locked cargo-watch

# 開発モードでの実行（ホットリロード）
CMD ["cargo", "watch", "--why", "-x", "check", "-x", "run --bin rust-start-app", "-w", "src", "-w", "Cargo.toml"]
