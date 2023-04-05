# Development

## Repository Structure

This repository is split into multiple projects at the root folder. Each project has its own dependencies and tools used to build, test, and ship different parts of the repository. For example, a Rust environment for the compiler, a Python environment for documentation, and a NodeJS environment for linters. This allows us to implement different binaries, APIs, and internal tools/scripts, and package/version them together, while having minimal inter-dependencies.

All dependencies should have exact full versions, and we can rely on tooling to automatically upgrade it over time. It allows us to have perfectly reproducible builds for every commit, [a critical aspect](https://github.com/dotnet/designs/blob/40794be63ecd8b35e9596412050a84dedd575b99/accepted/2020/reproducible-builds.md) for compilers, and developer tools in general.

## Folder Structure

Currently, the repository has the following projects:

- `./.devcontainer`: devcontainer image that is self-contained, and has all the dependencies needed to develop, build, test, and ship the project.
- `./crates`:
  - `codegen/`: code generation scripts.
  - `solidity/`: Solidity language inputs and outputs.
- `./documentation`: used to host the language specification, and any future documentation for the project.
- `./scripts`: self-contained bash scripts to setup, build, test, run, and release all code in this repository.

## Dev Containers

To make the developer experience as seamless and consistent as possible, we recommend using the VS Code [devcontainer](https://github.com/NomicFoundation/slang/tree/main/.devcontainer) included in this repository. It is a light image that has the minimum required tools to build this project. If you are not familiar with containerized development, I recommend taking a look at [the official VS Code guide](https://code.visualstudio.com/docs/remote/containers). Using a devcontainer allows us to quickly setup/teardown the environment, and install/setup different dependencies for different projects, without polluting the local environment. In the future, it will enable us to include Windows and Mac OS specific images for cross-platform testing.

If you would like to still develop outside a container, this should still be possible, as the CI will guarantee that your changes are safe. We intend to keep the images to a bare minimum, and install most dependencies through scripts you can run locally. However, using a common development container means sharing and standardizing useful settings and extensions for the editor (VS Code), the terminal (zsh), and any other tools.

In the future, if we decide to enable code spaces, we can have a 1-click button to create and warm up a powerful dev machine to use in seconds, and running completely remote in a browser tab. It will make it trivial to switch between different versions and branches, or even use and debug multiple ones at the same time from different tabs.

## Hermit

To install language-specific binaries and packages, we use [Hermit](https://cashapp.github.io/hermit/), which installs all tools only when it is first needed/invoked, so you can quickly setup and build different projects quickly. It also takes care of updating your `$PATH` as you `cd` in and out of different projects, to make sure you are using the right tools every time. Follow [this guide](https://cashapp.github.io/hermit/usage/get-started/) to install it locally to your machine, or simply build any included project, and it will bootstrap itself if it is missing.

## Build Scripts

To ensure consistency, and a good experience for first-time developers, all build/test/run/debug commands should be written, versioned, and documented inside the `scripts/` folder. This means that any dev instructions are well documented, versioned, and verified/executed with every build. Scripts should never assume or use `cwd`, but activate their own hermit environment and run all other commands using repo-relative paths. This allows reusing/invoking across projects.