# Get the version from Cargo.toml
VERSION=$(shell grep '^version =' Cargo.toml | sed -E 's/version = "(.*)"/\1/')

# Get the short commit hash from git
COMMIT_HASH=$(shell git rev-parse --short HEAD)

# Image name
IMAGE_NAME=can-i-connect

# Default target: build the Docker image
build:
	@echo "Building Docker image with tag $(IMAGE_NAME):$(VERSION)-$(COMMIT_HASH)"
	docker build -t $(IMAGE_NAME):$(VERSION)-$(COMMIT_HASH) .

# Target to print the version and commit hash (for debugging)
version:
	@echo "Cargo.toml version: $(VERSION)"
	@echo "Git commit hash: $(COMMIT_HASH)"

# Target to clean up the Docker image
clean:
	@echo "Removing Docker image with tag $(IMAGE_NAME):$(VERSION)-$(COMMIT_HASH)"
	docker rmi $(IMAGE_NAME):$(VERSION)-$(COMMIT_HASH) || true
