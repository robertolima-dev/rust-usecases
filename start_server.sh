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

PROJECT_PATH="/Users/robertolima/Documents/projects/rust/rust-usecases"

cd $PROJECT_PATH
docker-compose up -d zookeeper_rust kafka_rust redis_rust elasticsearch_rust kibana_rust redis-commander_rust mongodb_rust

# Run migrations antes de iniciar o servidor
echo "🛠️  Rodando migrations..."
sqlx migrate run

# Rodar todos os testes
echo "🧪 Rodando testes unitários e de integração..."
cargo test -- --nocapture

echo "✅ Testes concluídos!"

# Inicia o cargo watch com hot reload
echo "🚀 Iniciando servidor com hot reload (cargo watch)..."
cargo watch -q -c -w src/ -x "run -- api"
# cargo watch -q -c -w src/ -x run
