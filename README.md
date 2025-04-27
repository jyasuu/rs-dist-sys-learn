# rs-dist-sys-learn

wget https://github.com/jepsen-io/maelstrom/releases/download/v0.2.4/maelstrom.tar.bz2
tar -xvf maelstrom.tar.bz2

maelstrom/maelstrom test -w echo --bin maelstrom/demo/ruby/echo.rb --time-limit 5