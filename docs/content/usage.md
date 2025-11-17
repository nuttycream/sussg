---
title: Usage
description: usage
is_post: true
---

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
- templates -> for `mustache` templates
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
