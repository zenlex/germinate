```
        @@
@@     _/@
  \_  /
    \/          @
    |          \|/
 --\@/germinate\@/--
```

# Description

A project launchpad to accelerate from idea to first line of code. 

Mostly an excuse to learn Rust better by building something I might use. 

Currently prototyping around stacks I currently use personally, but building with a mind for future extensibility. 

Most components of a stack will be configurable via TOML file and templates will be able to be included for any manifests, config files, etc. for that stack

The common components across many stacks (databases, linters, formatters, etc) will be configured via CLI dialogue and installed along with the other project dependencies. 

# Concept/Usage (eventually --WIP--)
- run `germinate [project name]`
- answer questions
- germinate:
   - creates folders
   - installs dependencies
   - initializes source control
