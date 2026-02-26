# Define the service name from your docker-compose.yml
SERVICE_NAME=app

.PHONY: run build stop clean

# The main command to run the app interactively
run:
	docker compose run --rm $(SERVICE_NAME)

# Force a rebuild (useful if you change Cargo.toml or dependencies)
build:
	docker compose build

# Stop any running containers for this project
stop:
	docker compose down

# Clean up the target volume if things get messy
clean:
	docker compose down -v