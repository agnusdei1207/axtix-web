# Rust
## actix-web 연습장

- sea orm 사용
- postgres 연결

- Docker Run
  DOCKER_RUN_CLI=docker run -d \
   --name postgres_db \
   -e POSTGRES_USER=rest \
   -e POSTGRES_PASSWORD=rest \
   -e POSTGRES_DB=rest \
   -p 5432:5432 \
   -v postgres_data:/var/lib/postgresql/data \
   postgres:latest

- Sea orm cli
  CREATE_ENTITY_CLI=sea-orm-cli generate entity -u postgres://rest:rest@localhost:5432/rest -o entity/src
