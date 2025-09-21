CARGO := cargo

# server run
start:
	@echo "🚀 Iniciando servidor..."
	$(CARGO) run

# server watch run
dev:
	@echo "👁️  Monitorando arquivos para reinicialização automática..."
	cargo-watch -x run

debug:
	@echo "👁️  Monitorando arquivos para reinicialização automática..."
	RUST_LOG=debug cargo-watch -x run

trace:
	@echo "👁️  Monitorando arquivos para reinicialização automática..."
	RUST_LOG=trace cargo-watch -x run