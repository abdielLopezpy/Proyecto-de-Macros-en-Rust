# Makefile

# El comando por defecto será `watch` para ver los cambios en tiempo real
all: watch

# Comando para ejecutar cargo expand
expand:
	cargo expand

# Comando para monitorizar cambios en los archivos y ejecutar cargo expand automáticamente
watch:
	@echo "Monitoring files for changes..."
	@find src/ -name "*.rs" | entr -r make expand

# Limpia los archivos compilados (opcional)
clean:
	cargo clean

.PHONY: all expand watch clean
