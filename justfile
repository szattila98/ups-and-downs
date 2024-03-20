_default:
  @just --list --unsorted

_check-app app:
    @if ! [ -x "$(command -v {{app}})" ]; then \
        echo "\033[1;31m{{app}} is not installed ✘!\033[0m"; \
        exit 1; \
    fi

install-cargo-tooling:
    # Installing (or if installed updating) cargo tooling
    @just _check-app cargo
    @cargo install cargo-watch cargo-audit cargo-llvm-cov cargo-edit sqlx-cli cocogitto 

add-git-hook:
    # Adding Cocogitto hook to local repository
    @just _check-app cog
    @cog install-hook pre-push

watch:
    @yarn && yarn tauri dev
