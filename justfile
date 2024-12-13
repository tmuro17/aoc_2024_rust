set fallback

_default:
  @just --list

test *FLAGS:
  cargo insta test --test-runner nextest {{FLAGS}}
