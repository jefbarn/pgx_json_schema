ARG POSTGRES_VERSION=13
ARG PGX_JSON_SCHEMA_VERSION=main

# Use build stage to keep all the souce code out of the final image
FROM postgres:$POSTGRES_VERSION as build
ARG POSTGRES_VERSION
ARG PGX_JSON_SCHEMA_VERSION

# Update postgres and install libraries
RUN apt-get update \
  && apt-get install -y --no-install-recommends \
    ca-certificates \
    postgresql-$POSTGRES_VERSION \
    postgresql-server-dev-$POSTGRES_VERSION \
    build-essential \
    curl \
    pkg-config \
    libssl-dev \
    libreadline-dev \
    zlib1g-dev \
    flex \
    bison \
    libxml2-dev \
    libxslt-dev \
    libxml2-utils \
    xsltproc \
  && rm -rf /var/lib/apt/lists/*

USER postgres
SHELL ["/bin/bash", "-c"]

# Install Rust
# https://www.rust-lang.org/tools/install
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="~/.cargo/bin:${PATH}"

# Install PGX
WORKDIR $HOME
RUN cargo install cargo-pgx
RUN cargo pgx init --pg$POSTGRES_VERSION /usr/bin/pg_config

# Download this repo
RUN mkdir $HOME/pgx_json_schema
WORKDIR $HOME/pgx_json_schema/
RUN curl -L "https://github.com/jefbarn/pgx_json_schema/archive/${PGX_JSON_SCHEMA_VERSION}.tar.gz" \
   | tar -xz --strip-components=1

# Build and install the extension package
RUN cargo pgx package


# Now create the final image
FROM postgres:$POSTGRES_VERSION
ARG POSTGRES_VERSION

COPY --from=build $HOME/pgx_json_schema/target/release/pgx_json_schema-pg$POSTGRES_VERSION /
