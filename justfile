

# runs custom script (UPDATES GIT REPO:)
up: 
	update_repo.sh

# runs tailwindcli (HOT RELOAD)
watch:
	./tailwindcss -i styles.css -o output.css --watch
	
# runs trunk serve (HOT RELOAD)
serve: 
	trunk serve

# runs cargo tauri dev (NO HOT RELOAD)
dev:
	cargo tauri dev


### Installations

# Installs tailwindcss cli (LINUX)
tail-cli:
	curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64

