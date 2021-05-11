#!/usr/bin/env sh
cargo clippy -- \
  -W clippy::pedantic \
  -A clippy::wildcard-imports \
  -A clippy::must_use_candidate \
  -A clippy::missing_errors_doc \
  -A clippy::missing-panics-doc \
  -A clippy::or_fun_call \
  -A clippy::module_name_repetitions \
  -A clippy::new_ret_no_self
