# NX Monorepo Blueprint

<a alt="Nx logo" href="https://nx.dev" target="_blank" rel="noreferrer"><img src="https://raw.githubusercontent.com/nrwl/nx/master/images/nx-logo.png" width="45"></a>

A monorepos blueprint powered by [Nx](https://nx.dev) and [@monodon/rust](https://github.com/nrwl/monodon) for efficient development and build management.

## 🚀 Quick Start

### Prerequisites

- [Node.js](https://nodejs.org/) (v18 or later)
- [Rust](https://rustup.rs/) (latest stable)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)

### Installation

```bash
npm install

rustc --version
cargo --version
```

## 📁 Project Structure

```
├── apps/                    # applications
│   └── rust_forge_boilerplate/  # application
├── libs/                    # Shared libraries
│   └── errors/             # Error handling library
├── tools/                   # Development tools
│   ├── create-rust-app.js  # App generator script
│   └── rust_app_template/  # Template for new apps
└── dist/                   # Build outputs
```

## 🛠️ Development Commands

### Building Projects

```bash
#build all binary
npx nx run-many -t build

#build one binary
npx nx build rust_forge_boilerplate
```

### Testing

```bash
#all binary apps test
npx nx run-many -t test

#one binary applications test
npx nx test rust_forge_boilerplate
```

### Running Applications

```bash
npx nx run rust_forge_boilerplate
```

## 📦 Creating New Projects

### Generate a New Rust Library

```bash
npx nx g @monodon/rust:lib my-new-lib
```

### Generate a New Rust Application

```bash
npm run create:rust-app {your_app_name}
```

## 🔧 Available Scripts

- `npm run create:rust-app` - Interactive script to create new Rust applications

## 📊 Project Visualization

```bash
npx nx graph

npx nx affected:graph
```

## 🏗️ Build Targets

Each Rust project supports these targets:

- **build** - Compile the project
- **test** - Run unit tests
- **run** - Execute the binary (for applications)

## 🔍 Troubleshooting

### Common Issues

1. **Cargo.lock conflicts**: Run `cargo update` in the workspace root
2. **Build cache issues**: Clear with `npx nx reset`

### Useful Commands

```bash
# Clear Nx cache
npx nx reset

# Show project information
npx nx show project rust_forge_boilerplate

# List all projects
npx nx show projects
```
