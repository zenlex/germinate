```
        @@
@@     _/@
  \_  /
    \/          @
    |          \|/
 --\@/germinate\@/--
```

A launchpad for accelerating from idea ----->>> code. 

## Table of Contents
- [Description](#description)
- [Usage](#usage)
- [Options](#options)
- [Frontend](#frontend)
- [Database](#database)
- [Linting](#linting)
- [Testing](#testing)
- [Formatting](#formatting)
- [Containers](#containers-web-stacks-only)
- [Source Control](#source-control)
- [Customization](#customization)
- [Additional Template Files](#additional-template-files)
- [Installation](#installation)
- [Future Plans](#future-development-plans)

## Description
This project is mostly an excuse to learn Rust better by building something I might actually use. My other intent was to narrow some tooling choices and iterate on stacks I personally enjoy to 'dial them in' without having to remember that one package I liked for that one thing on that one project.  

My hope is that it will encourage more rapid prototyping of ideas by lowering the startup workload, but with more flexibility than just using starter repos. 

Germinate is built around stacks I use personally, but with decent foundations for extensibility. 

## Usage 
- `cd` into parent folder for new project
- run `germinate` (if not in PATH, use full path to binary)
- answer questions
- cd `project name` 
- start building! 

## Options
### Current stacks supported:
- Rust API
- TypeScript API
- Rust CLI tool
- TypeScript CLI tool

### Notes on Stacks 
- RSAPI - Minimal Axum Web framework with tokio runtime and tower for http
- TSAPI - Bun + Hono + Zod - Typesafe, minimal, fast, and built for the edges
- RSCLI + TSCLI - Rust and TS CLI tool setups with useful tools for command line interaction

_these stacks can be modified by [changing their `stack_template.toml` files](#customization)_
### Frontend
- Rust and Typescript API stacks include option to add templating (via Handlebars) or a Vue SPA with Vite. 

### Database
All 4 stacks offer DB client options. Currently supported:
- Postgres (Rust - sqlx or Diesel ORM, TypeScript - Slonik or Prisma ORM)
- Sqlite (Rust - sqlx or Diesel ORM, TypeScript - Better sqlite3 )
- MongoDB (Rust - mongodb driver(no ORM option), TS - mongodb or Mongoose ODM)

*Note - selecting a database does not install the database platform, only the client unless you select [Docker containers in the options](#containers)

### Linting
- All TypeScript stacks come with ESLint installed 
- All Rust stacks include clippy
- All Web Stacks (TSAPI & RSAPI) include StyleLint if a frontend is enabled

### Testing
- All JS/TS stacks are currently built with bun which has a native test runner out of the box. Rust also has native test running support via `cargo test`.  
- The optional SPAs for the web stacks both leverage Vite/Vue's create flow where you can select additional front end testing options (Vitest + Playwright recommended)

### Formatting
- ESLint is installed for the TypeScript stacks and can be selected as the formatter in your IDE of choice.
- rustfmt is native to the Rust ecosystem and the recommended choice for the Rust stacks. 

### Containers (Web stacks only) 
- Selecting 'Yes' for the containers option with a web stack will copy the `/docker` folder from `GERMINATE_PATH/templates/{stack}/` to the project folder after injecting config variables.

This will result in:
- A `docker-compose.yml` with services for:
  - `web` (the main app), 
  - `frontend` (spa option only) running the Vue/Vite app
  - `db` and `db_test` services for the selected database 
  - `database` and `database_test` docker volumes for persistence 
  - Dockerfiles and entrypoint scripts as starting points for dev and production builds

### Source Control 
- A git repositiory is automatically initialized at the project root and an initial commit made post project setup. `.gitignore` files can also be customized in the `templates` folders. 


## Customization
- All customizable config options for a stack are located in `templates/{stack}/stack_template.toml`
- Currently adding stacks or core platforms/tools (db, linter, formatter, package manager, etc.) is not supported. 
- You may describe a folder structure you'd like created within the root project folder using the `subfolders` key in the `stack_template.toml`
- Scripts for TS stacks are easily customized by modifying the `stack_template.toml` file for either `templates/tsapi` or `templates/tscli` and adding to (or creating) the scripts.npm table
```
[[scripts.npm]]
scriptname="run these commands"
```
- Additional dependencies may be added to any of the stacks by modifying their `stack_template.toml` file and following the patterns:
```
[[deps.cargo or deps.npm]]
name = {package_name} (required)
version = "1.0" (optional - defaults to 'latest' if not provided)
then = [["commandA", "arg1", "arg2"], ["commandB", "arg1", "arg2"]] (optional - these will be run after the install command for the package)
```

### Additional Template files
You may add files to your `templates/[stack]/` folder to be included on new projects. 
- Any folders/files in the `before_install` subfolder for a stack are copied to the new project folder prior to running the dependency install commands. 
- Any folders/files in the `after_install` subfolder for a stack are copied to the new project folder after running the dependency install commands. 

## Installation
_(Installer / docker image coming some day...)_

### Building from source (Rust / Cargo required)
- Clone repo
- Modify/Run Build Script `build.sh`
  - modify the `GERMINATE_PATH` and `BUILD` arguments
  - run `sh build.sh` - this script will place the built binary at `GERMINATE_PATH`/germinate and the stack template files at `GERMINATE_PATH/templates/`
  - if `BUILD=release` then cargo will build a release binary - you should use this if you don't plan on modifying the code.
  - *Make sure to add your `GERMINATE_PATH` location to your system PATH if you want to be able to call it from any parent folder* 

### Manual Build
- Run `cargo build` for a dev build or `cargo build --release` for a productio build if you're not planning to modify the app. 

- place the built binary from `/target/(release|debug)/germinate` wherever you'd like on your system, and copy the `/templates` folder to the same loation as the binary. 


*Everything comes with some amount of linting, formatting, and testing whether you like it or not ;)* 
## Future Development Plans
- [ ] Add better logging / progress indicators
- [ ] Make install / paths more configurable with cli options
- [ ] Extract stack list to make it extensible
- [ ] Extract database configs to make them extensible
- [ ] Extract templating engine config to make them extensible
- [ ] More robust package manager support TBD
- [ ] Build a TUI  
- [ ] Add other useful commandline flag support TBD
