<img src="https://raw.githubusercontent.com/nuttycream/sussg/main/docs/static/sussg.svg?sanitize=true" alt="sussy baka" width="200" align="right">

# sussg

**sussg** (pronounced sus-gee like Sasuke the guy who commited fratricide in
that one Japanese Animation) is a super ultimate static site generator that is
honestly just a wrapper for `pulldown-cmark` and `minijinja`. If you want a
better generator, please use [Zola](https://www.getzola.org).

If you're still here, then this is a no frills/barebones static site generator.
While other SSG's tout bloated features like cdn support, image processing,
taxonomies, and decent programming. Realistically, you don't need all that and
let's not kid ourselves, there's not gonna be that many people visiting your
site.

Well then why not just build out your entire site in raw html? You might ask,
which I'll reply with: good question, because well uhhh

## Features

- Simple
- Ultimate

## Installation

- If you're on linux and use nixpkgs, then you're in luck! It's the only
  platform I support
- Just add this repo to your `inputs` like so:

```nix
inputs = {
  sussg.url = "github:nuttycream/sussg"
};

outputs = {
  sussg,
  ...
}:
...
```

- Then add the package to your shell:

```nix
packages = [
  sussg.packages.${system}.default
];
```

- `sussg` should now be available in your shell.

> Note: you can use cargo as well
> `cargo install --git https://github.com/nuttycream/sussg`

## Usage

### Commands

```sh
# Initialize the project in a directory
sussg init

# Generate the sites into the output directory (default: ./public)
sussg build

# Serves the project locally through config.toml port (default: 3030)
sussg serve
```

### Structure

`sussg init` should've created the some directories/files and a default
`config.toml` in your current directory.

- content -> write your Markdown content here
- templates -> for `minijinja` templates
- styles -> for css files
- static -> images, fonts, or any static file. This is similar to Zola's static
  folder, where it just copies anything found here to the output directory.

A finished project structure may look something like this:

```sh
.
├── config.toml
├── content
│   ├── index.md
│   └── projects
│       └── index.md
│       └── some_project.md
├── static
│   ├── fonts
│   │   └── w95fa
│   │       └── w95fa.woff
│   └── feet.jpeg
├── styles
│   ├── main.css
└── templates
    ├── base.html
    └── homepage.html
```
