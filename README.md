# Ersa - GPC/GPX Package Manager & Utility

![GitHub license](https://img.shields.io/github/license/zKiwiko/ersa)
![GitHub issues](https://img.shields.io/github/issues/zKiwiko/ersa)
![GitHub pull requests](https://img.shields.io/github/issues-pr/zKiwiko/ersa)
![GitHub last commit](https://img.shields.io/github/last-commit/zKiwiko/ersa)
![GitHub contributors](https://img.shields.io/github/contributors/zKiwiko/ersa)

# Overview

Ersa is a package manager and utility for the GPC/[GPX](https://github.com/zKiwiko/gpx) Scripting languages.

Ersa offers multiple tools for GPC/GPX developers, including:

- **Standard Library**: A collection of common functions and utilities for GPC/GPX scripts.
- **Package Management**: Install, update, and remove installed packages.
- **Script Management**: Create, edit, and manage GPC/GPX scripts and projects.
- **Script Transpilation**: Transpile GPX scripts into GPC scripts using a dedicated
  transpiler, [gpx](https://github.com/zKiwiko/gpx).
- **Script Building**: Build GPC scripts into a single file for easy distribution and project management.

## Standard Library

The standard library is a collection of common functions and utilities for GPC/GPX scripts.
Ersa provides *core* support for the [GPC & GPX Standard Library](https://github.com/zKiwiko/gpx-stdlib).
The Standard Library is optional to have installed, but it is highly recommended.

### Installation

```bash
ersa pkg --install core
```

## Package Management

Ersa provides a package manager for GPC/GPX scripts. You can install, update, and remove packages from any repository
from any creator. As long as their package follows the correct format.

### Installing a package

Run the following command to install a package.
The installed package will be installed in the `[install dir]/bin/lib` directory.

```bash
ersa pkg --install <package repo clone url>
````

```bash
ersa pkg --install https://github.com/zKiwiko/gpx-stdlib.git
```

### Updating a package

Run the following command to update a package.
Ersa will check the version that is currently installed by the package's `lib.json` file in its root, then check the
package's repository version from its `url` field.

If the version is different, Ersa will update the package to the latest version.

```bash
ersa pkg --update <package repo clone url>
````

```bash
ersa pkg --update https://github.com/zKiwiko/gpx-stdlib.git
```

### Removing a package

Run the following command to remove a package.
Ersa will remove the package from the `[install dir]/bin/lib` directory.

The parameter must match a package's name. If you arent aware of your packages name, then you can run the following
command to list all installed packages.

```bash
ersa pkg --list
```

Then, you can run the following command to remove a package.

```bash
ersa pkg --remove <package name>
````

```bash
ersa pkg --remove std
```

### Package Format

The package format is a simple JSON file that contains the following fields:

- `name`: The name of the package.
- `version`: The version of the package.
- `url`: The URL of the package's repository.
- `modules`: A short description of the package.

example:

```json
{
  "name": "std",
  "version": "1.0.0",
  "url": "https://github.com/zKiwiko/gpx-stdlib.git",
  "modules": [
    "",
    "",
    ""
  ]
}
```

### Listing installed packages

Run the following command to list all installed packages.
This will list all installed packages, their version, and their repository URL.

```bash
ersa pkg --list
```

Or you can list details about a specific package by running the following command.

```bash
ersa pkg --list <package name>
```

## Script Management

Ersa provides a script manager for GPC/GPX scripts. You can create, edit, and manage GPC/GPX scripts and projects.

### Creating a project

You can run the following command to create a new project.
You have the choice of a GPC or GPX project.

```bash
ersa project --create <project name> --lang <gpc|gpx> (--output <path>)
```

# Build from source

To build Ersa from source, you need to have the following dependencies installed:

- [Git](https://git-scm.com/)
- [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Next, clone the repository and build the project:

```bash
git clone https://github.com/zKiwiko/ersa.git
cd ersa
```

Then, run the following command to build the project:

```bash
cargo build --release
```