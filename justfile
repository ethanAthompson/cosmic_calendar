

# runs custom script (UPDATES GIT REPO:)
up: 
	update_repo.sh

# runs tailwindcli (HOT RELOAD)
watch:
	./tailwindcss -i styles.css -o ./dist/output.css --watch
	# runs tailwindcli (HOT RELOAD)
watch-now:
	./tailwindcss -i styles.css -o output.css --watch
	
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

	
### Installations

# Installs tailwindcss cli (LINUX)
tail-cli:
	curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64

