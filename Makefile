
.PHONY: help
help:
	@echo "Usage: make <target>"
	@echo
	@echo "Targets:"
	@echo "  help             -- show this help"
	@echo "  shell            -- start a nix development shell"
	@echo "  contracts-build  -- build the contracts"
	@echo "  frontend-build   -- build the frontend"

.PHONY: shell
shell:
	nix-shell

.PHONY: contracts-build
contracts-build:
	cd contracts/item; cargo contract build

.PHONY: contracts-test
contracts-test:
	cd contracts/item; cargo test

.PHONY: frontend-build
frontend-build:
	cd frontend; deno task build
