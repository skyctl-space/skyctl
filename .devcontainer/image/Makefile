REGISTRY=ghcr.io/skyctl-space
IMAGE=devcontainer
VERSION=25.05.0

TARGET=${REGISTRY}/${IMAGE}:${VERSION}

.PHONY: build publish login setup-buildx

# Ensure Buildx is set up
setup-buildx:
	docker buildx create --use --name multiarch-builder || true
	docker buildx inspect multiarch-builder --bootstrap

# Build multi-platform image
build:
	docker buildx build --platform linux/amd64,linux/arm64 -t ${TARGET} .

# Publish is now part of the build step since --push is used
publish:
	docker buildx build --platform linux/amd64,linux/arm64 -t ${TARGET} --push .

# Login to the registry
login:
	echo $(GITHUB_TOKEN) | docker login ghcr.io -u ddompe --password-stdin
