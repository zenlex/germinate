```
        @@
@@     _/@
  \_  /
    \/          @
    |          \|/
 --\@/germinate\@/--
```

# Description

A project launchpad to accelerate going from idea to first line of code. 

Mostly an excuse to learn Rust better by building something I might actually use. 
My hope is that it will encourage more rapid prototyping of ideas by lowering the startup workload, but with more flexibility than just using starter repos. Especially for smaller personal projects. 

Currently prototyping around stacks I use personally, but building with a mind towards future extensibility. 

Most components of a stack will be configurable via a simple TOML file and templates will be able to be included for any manifests, config files, Dockerfiles, etc. for that stack

The components common across multiple stacks (databases, linters, formatters, etc) will be configured via CLI dialogue and installed along with the other project dependencies. 

# Concept/Usage (eventually --WIP--)
- run `germinate [project name]`
- answer questions
  - which stack? 
  - database or not?
    - if database - which platform?
  - linter & formatter preferences
- germinate:
   - creates folders
   - installs dependencies
   - copies/hydrates any templates (Dockerfiles, etc)
   - initializes source control
