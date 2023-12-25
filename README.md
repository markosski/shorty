# shorty

Shorty is a simple application for shortcut lookups. Mainly built as a way to practice Rust but I also wanted an easy way to lookup VIM shortcuts.
It uses YAML to define shortcuts with following structure:

```yaml
- description: "Description of section"
  items:
  - shortcut:
      macos: cmd + a
      linux: ctrl + a
      default: ctrl + a
    description: does something
  - shortcut:
      default: ctrl + b
    description: does something
```

Shortcuts can be defined for specific operating system, e.g. macos, linux, windows. If there is no need to that, just a `default` can be provided.

## Usage

Note: I only tested this under Mac. It should probably work under Linux but not so sure about Windows.

Build locally
`cargo build`

Install as application on Mac
`cargo bundle`


## Initialize directory and copy cheatsheets into it.

`mkdir -p ~/.shorty/cache`

`cp cheatsheets/* ~/.shorty/cache`

<img width="912" alt="Screen Shot 2023-12-25 at 12 26 26 PM" src="https://github.com/markosski/shorty/assets/916909/5b25a073-1e6f-4314-8f0f-0dbec593a3de">


