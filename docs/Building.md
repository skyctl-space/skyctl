# Building the Code

## Code layout

The code is spread in 3 main git repos right now:
- [Skyctl repo](https://github.com/skyctl-space/skyctl): holds the code for the tauri backend and Vue frontend
- [Skydata repo](https://github.com/skyctl-space/skydata): holds the information for the stellarium. This is decouple from the front end
so we can update for older versions independently and also make the git repo easier to handle. This repo is cloned as a submodule 
from the main repo.
- [Stellarium-web fork](https://github.com/skyctl-space/stellarium-web-engine): this is a fork from the mainstream stellarium-web project
that includes our changes. This repo produces web-assembly: we fetch from a release during the build process

The project is organized as follows:

- **`src/`**: Contains the main Vue components, utilities, and assets.
- **`public/`**: Public assets served directly, such as images and fonts.
- **`src-tauri/`**: Tauri-specific files for building the desktop application.

## Build dependencies

In order to build the project several dependencies are requiered per platform. For development
proposes the main recommendation is to use the devcontainer based image published to the github container register `ghcr.io/skyctl-space/devcontainer`. There is a `.devcontainer/devcontainer.json` file that allows to open the repo with any devcontainer-compatible developer environment (easier one would be VSCode). The image is build for `amd64` and `arm64` platforms, so could be easily used on CI systems or local development, including Apple Silicon Macs.

The source code Dockerfile for the devcontainer image is found at `.devcontainer/image` and is the best reference for the updated dependencies.

## Cloning

Remember to clone recursively to pull the skydata:

```bash
git clone --recursive https://github.com/skyctl-space/skyctl.git
cd skyctl
```

## Building
Assuming that you either manually installed all the required dependencies or are running inside a devcontainer these are the steps to build the code:

To build a release locally:
```bash
pnpm tauri build
```

To start a development environment:
```bash
pnpm tauri dev
```
