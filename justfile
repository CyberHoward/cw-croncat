test_addrs := env_var_or_default('TEST_ADDR', `jq -r '.[].address' ci/test_accounts.json | tr '\n' ' '`)

set export
lint:
	cargo fmt --all && cargo clippy -- -D warnings
test:
	#!/bin/bash
	cargo test -- --nocapture 

build:
	#!/bin/bash
	set -e
	export RUSTFLAGS='-C link-arg=-s'
	cargo build --release --lib --target wasm32-unknown-unknown
deploy:
	#!/bin/bash
	chmod +x ./scripts/testnet/deploy.sh
	./scripts/testnet/deploy.sh -w # only wasm update
deploy-local:
	#!/bin/bash
	chmod +x ./scripts/local/deploy.sh
	./scripts/local/deploy.sh -w # only wasm update

juno-local:
	docker kill cosmwasm || true
	docker volume rm -f junod_data
	docker run --rm -d --name cosmwasm \
		-e PASSWORD=xxxxxxxxx \
		-e STAKE_TOKEN=ujunox \
		-e GAS_LIMIT=100000000 \
		-e MAX_BYTES=22020096 \
		-e UNSAFE_CORS=true \
		-p 1317:1317 \
		-p 26656:26656 \
		-p 26657:26657 \
		-p 9090:9090 \
		--mount type=volume,source=junod_data,target=/root \
		ghcr.io/cosmoscontracts/juno:v11.0.3 /opt/setup_and_run.sh {{test_addrs}}

deploy-local-reset:
	#!/bin/bash
	chmod +x ./scripts/local/deploy.sh
	./scripts/local/deploy.sh -w  -c #  wasm update & container update

optimize:
	docker run --rm -v "$(pwd)":/code \
		--mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		--platform linux/amd64 \
		cosmwasm/workspace-optimizer:0.12.10

gen-schema:
	./scripts/schema.sh

gen-typescript:
	yarn --cwd ./typescript install --frozen-lockfile
	yarn --cwd ./typescript build
	yarn --cwd ./typescript codegen

cbuild:
    @echo 'Changing manifest symlinks to use versioning / relative paths:'
    {{justfile_directory()}}/scripts/set-manifests-regular.sh
    @echo 'Building CronCat contracts:'
    {{justfile_directory()}}/scripts/cargo-build.sh
    @echo 'Changing manifest symlinks to point to Cargo workspaces:'
    {{justfile_directory()}}/scripts/set-manifests-workspaces.sh

cdeploy:
    #!/bin/bash
    cd {{justfile_directory()}}/scripts/deployment
    npm run go-regular

schema: gen-schema gen-typescript

all: lint build test schema
