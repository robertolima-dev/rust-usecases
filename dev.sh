#!/bin/bash

# Parar o script se houver erro
set -e

# Verificar se .env existe e carregar
if [ -f .env ]; then
  echo "📦 Carregando variáveis de ambiente do .env..."
  export $(grep -v '^#' .env | xargs)
else
  echo "⚠️  Arquivo .env não encontrado. Continuando sem variáveis..."
fi

# Run migrations antes de iniciar o servidor
echo "🛠️  Rodando migrations..."
sqlx migrate run

# Inicia o cargo watch com hot reload
echo "🚀 Iniciando servidor com hot reload (cargo watch)..."
cargo watch -q -c -w src/ -x run
