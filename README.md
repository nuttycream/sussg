<img src="https://raw.githubusercontent.com/nuttycream/sussg/main/docs/static/sussg.svg?sanitize=true" class="sussy-baka" alt="sussy baka" width="150" align="right">

# sussg <img src="https://img.shields.io/github/v/release/nuttycream/sussg">

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

### Features

- Simple
- Ultimate
- Templating with [MiniJinja](https://docs.rs/minijinja/latest/minijinja/)
- CommonMark compliant using [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark/)
  - Plus some extra goodies such as GFM tables, footnotes, wikilinks, etc.
- Plugin Support
- Browser live-reload

### For who?

Who knows, really. The name birthed the project and it sorta became my personal static site generator. If you want a fairly low-cortisol static site generator, then this might work for you. Though, it does require a lot of upfront work from the end user. As it doesn't have any built-in templates, to use, other than this `/docs` site example.

## Install

### Cargo

`sussg` can be downloaded and installed directly through cargo:

```sh
cargo install --git https://github.com/nuttycream/sussg
```

### Nix

`sussg` also exposes itself through nix:

```nix
# as a flake input:
inputs = {
    sussg.url = "github:nuttycream/sussg";
};
```

Or if you want to use `sussg` in the current shell session:

```sh
nix shell github:nuttycream/sussg
```

### Cachix

`sussg` also has a cache available through Cachix. Add the following to your NixOS configuration to avoid lengthy rebuilds and fetching extra build-time dependencies:

```nix
{
  nix.settings = {
    substituters = [ "https://nuttycream.cachix.org" ];
    trusted-public-keys = [
      "nuttycream.cachix.org-1:x8URTEpNkAQpFCf0oW5nPDJGI4Q9E8HtAO66JBjSWlY="
    ];
  };
}
```

Or if using the Cachix CLI outside a NixOS environment:

```sh
cachix use nuttycream
```

### Manual Compile

```sh
git clone https://github.com/nuttycream/sussg
cd sussg
cargo install --path .
```

That should drop a `sussg` binary on your `$PATH`. Confirm with:

```sh
sussg --help
```

## Init

To initialize the site, run:

```sh
sussg init
```

This should create files/folders in the current directory in this layout:

```sh
.
├── config.toml
├── content/      # markdown content
├── plugins/      # re-usable html snippets
├── static/       # images, fonts, anything that needs to be copied
├── styles/       # .css files
└── templates/    # minijinja .html templates
```

You can use `-p some/path` to generate into a specific directory.
Use the navbar on the left to jump to learn more about a specific folder.

## Build

```sh
sussg build
```

Builds everything in `content/` into static html and writes it to `public/`
or wherever your `output_dir` is set to.

| flag                | what it does                                           |
| ------------------- | ------------------------------------------------------ |
| `-p, --path PATH`   | site root, defaults to `./`                            |
| `-o, --out PATH`    | override the output directory from config              |
| `-l, --local`       | force `site_url` to `/`                                |
| `-d, --drafts BOOL` | include pages with `draft = true` in their frontmatter |

## Serve

```sh
sussg serve
```

Builds the site, watches for file changes, and serves the result on
`localhost:{{port}}` with browser auto-reload.

> `serve` always builds in local mode (`site_url = "/"`) and turns drafts on by
> default. Pass `-d false` to hide them.

| flag                | what it does                            |
| ------------------- | --------------------------------------- |
| `-p, --path PATH`   | site root                               |
| `-o, --out PATH`    | override the output directory           |
| `--port PORT`       | port to listen on, defaults to `3030`   |
| `-d, --drafts BOOL` | include drafts, defaults to `true` here |

The watcher polls `templates/`, `styles/`, `content/`, `static/`, `plugins/`,
and `config.toml` for changes. When something changes, the site should rebuild and
your browser should auto reload via SSE.

## Config

Site wide config settings live in `config.toml` at the root of your site. Every
value has a default, so you only set what you actually want to change.

```toml
[general]
url = "/"
output_dir = "public"
drafts = false

[style]
main = ["main"]

[template]
base = "base"

[serve]
port = 3030
drafts = true
```

A few notes on what these actually do:

`general.url` is what gets prepended to internal links via `{{ site_url }}`.
Set it to your full URL `https://some.site.jpeg/`.
But both `serve` and `build -l` ignore this and force `/`.

`style.main` is a list of stylesheet names (without `.css`) that get linked
on every page automatically. Any extra CSS in `styles/` is still copied to
`public/`, it just won't be auto-linked unless a page asks for it.

`template.base` picks which file in `templates/` is used when a page doesn't
override it.

## Frontmatter

Each page should declare metadata `sussg` codeblock to properly be registered. This can be placed anywhere within the page.

<pre class="language-toml"><code>```sussg
type = "frontmatter"
title = "installing sussg"
date = "2024-08-12"
description = "how to get sussg working"
draft = false
```</code></pre>

Possible args:

| field         | type            | what it does                                             |
| ------------- | --------------- | -------------------------------------------------------- |
| `title`       | string          | page title (required)                                    |
| `description` | string          | short blurb, exposed in templates                        |
| `author`      | string          | self-explanatory                                         |
| `date`        | string          | format up to you, used for sorting/display               |
| `draft`       | bool            | if true, page is skipped during build unless `-d` is set |
| `template`    | string          | use a different template than the configured base        |
| `styles`      | array of string | extra stylesheets to link (names without `.css`)         |
| `is_archived` | bool            | metadata flag, exposed via `frontmatter` in templates    |

## Content

Anything in `content/` ending in `.md` becomes a page. URL routing works like
this:

| source                     | url               |
| -------------------------- | ----------------- |
| `content/index.md`         | `/`               |
| `content/posts/index.md`   | `/posts/`         |
| `content/posts/install.md` | `/posts/install/` |
| `content/doofus/bleh.md`   | `/doofus/bleh/`   |

Subdirectories also become `sections`. Anything under `content/posts/` is
grouped into the `posts` section, accessible from any template as
`{{ sections.posts }}`. Same for `content/notes/`, `content/projects/`, and
so on.

## Templates

Templates live in `templates/` as `.html` files and use
[minijinja](https://docs.rs/minijinja). Every template gets the same context:

| variable      | what it is                                          |
| ------------- | --------------------------------------------------- |
| `title`       | the page's title from frontmatter                   |
| `content`     | the rendered page html                              |
| `frontmatter` | the full frontmatter struct                         |
| `headings`    | list of `{ level, text, id }` for building a TOC    |
| `sections`    | map of section name → list of pages in that section |
| `most_recent` | map of section name → its most recent page          |
| `site_url`    | the configured `general.url` (or `/` in local mode) |

`site_url` is not usually required to be set, it's meant to be overridden
and used when your site domain is not root, for example `https://nuttycream.github.io/sussg` requires it and variables referencing root need to have the `{{ site_url }}` otherwise url breaks and would resolve to `https://nuttycream.github.io/`.

An example minimal `templates/base.html`:

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <title>{{ title }}</title>
  </head>
  <body>
    <div class="content">{{ content | safe }}</div>
  </body>
</html>
```

A page can override the template by setting `template = "post"` in its
frontmatter, which would resolve to `templates/post.html`. Templates can use
each other via minijinja's `{% extends %}` / `{% include %}` like normal.

### Building a table of contents

In your templates, the `headings` variable gives you every heading on the page with its `level`, `text`, and `id`:

```html
<nav class="toc">
  {% for h in headings %}
  <a href="#{{ h.id }}" class="{{ h.level }}">{{ h.text }}</a>
  {% endfor %}
</nav>
```

### Listing posts in a section

Anything under `content/posts/` is grouped into `sections.posts`. You can loop over them:

```html
<ul>
  {% for post in sections.posts %}
  <li>
    <a href="{{ post.url }}">{{ post.title }}</a>
    {% if post.date %}<time>{{ post.date }}</time>{% endif %} {% if
    post.description %}
    <p>{{ post.description }}</p>
    {% endif %}
  </li>
  {% endfor %}
</ul>
```

> Note: a reminder that anything within content/ can be turned into a "section". You can for example, have `content/stuffs/` which would be grouped as `section.stuffs`

## Styles

CSS files in `styles/` get copied as-is to `public/`. The `[style] main` list
in your config decides which ones are auto-linked into every page. Pages can
ask for extras via the frontmatter `styles` field.

At the moment, `sussg` can only support `.css` files.

## Static

Anything in `static/` gets copied verbatim to the root of `public/`. So
`static/favicon.ico` ends up at `/favicon.ico`, `static/models/Astronaut.glb`
at `/models/Astronaut.glb`. Reference these with `{{ site_url }}` prefixed in
templates so they keep working at any deploy URL.

## Plugins

A plugin is a minijinja template you keep in `plugins/`. Inside any markdown
page you can invoke it with a `sussg` codeblock, and the block gets replaced
with the rendered output of that template.

For example:

<pre class="language-toml"><code>```sussg
type = "plugin"
name = "3d_viewer"
src = "models/Astronaut.glb"
alt = "astronaut"
```</code></pre>

This says: find `plugins/3d_viewer.html`, render it with these args. Inside
that template, everything except `type` and `name` is available under `args`:

```html
<model-viewer
  alt="{{ args.alt }}"
  src="{{ site_url }}{{ args.src }}"
  shadow-intensity="1"
  camera-controls
  touch-action="pan-y"
>
</model-viewer>
```

Plugin templates have the same context as page templates (`title`,
`frontmatter`, `sections`, `site_url`, etc.), plus the `args` table.

## Deploy

### GitHub Pages

At the moment, I only know how to deploy with GitHub actions, I'm sure you can infer how to do it on other providers. If you want to add it here, open up a PR please!

I have no marketplace action you can use, though it's pretty straightforward:

```yaml
on:
  workflow_dispatch:
  push:
    branches:
      - main

permissions:
  contents: read
  pages: write
  deployments: write

jobs:
  build:
    name: build site
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5

      - name: install sussg
        run: cargo install --git https://github.com/nuttycream/sussg

      - name: build site
        run: sussg build

      - name: upload site files as artifact
        id: deployment
        uses: actions/upload-pages-artifact@v3
        with:
          path: ./public

  deploy:
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    runs-on: ubuntu-latest
    needs: build
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

You can pass in any [relevant args](#build) to `sussg build`. For example, to build with drafts, a specific folder, and a custom output path.

```yaml
jobs:
  build:
    name: build site
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5

      - name: install sussg
        run: cargo install --git https://github.com/nuttycream/sussg

      - name: build site
        run: sussg build --drafts true -p path/to/dir/ -o ./path/to/public

      - name: upload site files as artifact
        id: deployment
        uses: actions/upload-pages-artifact@v3
        with:
          path: ./path/to/public
```

### Nix

You can deploy using nix and a pre-built binary with Cachix:

```yaml
on:
  workflow_dispatch:
  push:
    branches:
      - main

permissions:
  contents: read
  pages: write
  deployments: write
  id-token: write

jobs:
  build:
    name: build site
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5
      - uses: cachix/install-nix-action@v25
        with:
          nix_path: nixpkgs=channel:nixos-unstable

      - name: Install Cachix
        uses: cachix/cachix-action@v14
        with:
          name: nuttycream

      - name: build site
        run: nix run github:nuttycream/sussg -- build

      - name: upload site files as artifact
        id: deployment
        uses: actions/upload-pages-artifact@v3
        with:
          path: ./public

  deploy:
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    runs-on: ubuntu-latest
    needs: build
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

References:

- [nix.dev](https://nix.dev/guides/recipes/continuous-integration-github-actions.html)

## Attribution

Most of the work done here were pulled from examples, but I want to specifically attribute the majority of the functionality to:

- [`pulldown-cmark`](https://github.com/pulldown-cmark/pulldown-cmark/) - converting markdown to HTML
- [`minijinja`](https://github.com/mitsuhiko/minijinja) - super advance templating with minimal deps

<br>
<br>
<hr>

> The frontmatter can be anywhere, for example here at the bottom of the page, hiding, from YOU! and so it doesnt muddy up the README.md. You can't see what I'm talking about in the docs site, but the GH readme should show exactly what I'm talking about.

```sussg
type="frontmatter"
title="sussg"
```
