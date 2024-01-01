
## Compile Targets
build-windows:
	cargo tauri build --target i686-pc-windows-msvc

## Compile Targets 2
build-deb:
	cargo tauri buid


build-git:
	trunk build --release --public-url cosmic_calendar/	

# runs custom script (UPDATES GIT REPO:)
up: 
	update_repo.sh

# runs tailwindcli (HOT RELOAD)
watch:
	./tailwindcss -i ./styles/input.css -o ./styles/output.css

# runs trunk serve (HOT RELOAD)
serve: 
	trunk serve

# runs cargo tauri dev (NO HOT RELOAD)
dev:
	cargo tauri dev

## Deployments

# builds project, Cleans & Builds (RELEASE)
build:
	trunk clean && trunk build --release

# restarts zone project
restart:
	cargo shuttle project restart --name zone

# Deploys zone project
deploy:
	cargo shuttle deploy --name zone --allow-dirty

deploy2:
	cargo shuttle deploy --name zone --allow-dirty --no-test

	
### Installations

# Installs tailwindcss cli (LINUX)
tail-cli:
	curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64

