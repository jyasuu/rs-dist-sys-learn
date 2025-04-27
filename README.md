# rs-dist-sys-learn

## target

https://fly.io/dist-sys/1/

## scripts

```sh
wget https://github.com/jepsen-io/maelstrom/releases/download/v0.2.4/maelstrom.tar.bz2
tar -xvf maelstrom.tar.bz2

maelstrom/maelstrom test -w echo --bin maelstrom/demo/ruby/echo.rb --time-limit 5

maelstrom/maelstrom test -w echo --bin target/debug/rs-dist-sys-learn --time-limit 5

```