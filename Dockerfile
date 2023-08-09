#-----PREPARE STAGE-----
FROM lukemathwalker/cargo-chef as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

#-----PLAN STAGE-----
FROM chef as planner
COPY . .
# compute a lock-like file for our project 
RUN cargo chef prepare --recipe-path recipe.json

#-----BUILD STAGE-----
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# build project dependencies, not our application
RUN cargo chef cook --release --recipe-path recipe.json

# Up to this point, if our dependencies tree stays the same,
# all layers should be cached
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin zero2prod

#-----RUN STAGE-----
FROM debian:bullseye-slim as runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  # Clean up
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*
  
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY --from=builder /app/configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]
