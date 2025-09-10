---
title: Installation
description: install
is_post: true
---

## Installation

- If you're on linux and use nixpkgs, then you're in luck! It's the only
  platform I support
- Just add this repo to your `inputs` like so:

```nix
inputs = {
  ssusg.url = "github:nuttycream/sussg"
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
