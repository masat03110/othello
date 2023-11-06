all: run

# `make`コマンドで実行するターゲットを指定
run:
	RUSTFLAGS='-C target-cpu=native' cargo run --release -- -H "localhost" -p 3000 -n AI

clean:
	cargo clean

# ターゲットを実行するためのエイリアスを設定
.PHONY: all run