# just commands for actix-web-datetime
# note: these only work in Powershell

set shell := ["powershell", "-c"]

dev:
  cargo watch -x clippy -x run
