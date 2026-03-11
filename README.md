# Verification Badge with Auths

This example demonstrates how to add a "Verified by Auths" badge and verification widget to your project using [Auths](https://github.com/auths-dev/auths).

## Verification Status

[![Verify Commits](https://github.com/auths-dev/example-verify-badge/actions/workflows/verify-commits.yml/badge.svg)](https://github.com/auths-dev/example-verify-badge/actions/workflows/verify-commits.yml)

## Quick Start

```bash
# 1. Fork this repo
# 2. Enable GitHub Pages (Settings > Pages > Source: GitHub Actions)
# 3. Push to main — the badge site deploys automatically
```

## What's Included

| Path | Purpose |
|------|---------|
| `docs/index.html` | Live verification widget demo page (all 3 display modes) |
| `docs/badge.svg` | Static "Verified by Auths" badge for README embedding |
| `.github/workflows/verify-commits.yml` | CI commit verification |
| `.github/workflows/deploy-badge-site.yml` | Deploy verification page to GitHub Pages |

## Embedding the Badge

### Static Badge (works everywhere)

Add this to your README:

```markdown
[![Commits Verified by Auths](https://your-org.github.io/your-repo/badge.svg)](https://your-org.github.io/your-repo/)
```

### Verification Widget (GitHub Pages / custom sites)

Add the `<auths-verify>` web component to your HTML page:

```html
<script type="module" src="https://unpkg.com/@auths-dev/verify/dist/auths-verify.mjs"></script>

<!-- Badge mode (compact) -->
<auths-verify repo="your-org/your-repo" forge="github" mode="badge"></auths-verify>

<!-- Detail mode (expanded) -->
<auths-verify repo="your-org/your-repo" forge="github" mode="detail"></auths-verify>

<!-- Tooltip mode (hover to expand) -->
<auths-verify repo="your-org/your-repo" forge="github" mode="tooltip"></auths-verify>
```

### Important Note

The `<auths-verify>` web component does **NOT** render in GitHub README files. GitHub strips custom HTML elements and `<script>` tags from Markdown. The widget works on:

- GitHub Pages sites
- Personal/project websites
- Documentation platforms (Docusaurus, MkDocs, etc.)
- Any HTML page you control

For GitHub READMEs, use the static SVG badge instead.

## Widget Display Modes

| Mode | Description |
|------|-------------|
| `badge` | Compact badge showing verified/unverified status |
| `detail` | Expanded view with signer info and verification details |
| `tooltip` | Badge that expands on hover to show details |

## How It Works

1. The `<auths-verify>` widget loads a WASM module that performs cryptographic verification in the browser
2. It fetches the repository's signing data from `refs/auths/` in the git repo
3. Verification happens entirely client-side — no backend required
4. The static SVG badge is a simple image for environments that don't support JavaScript

## Prerequisites

- GitHub Pages enabled on your repository
- [Auths CLI](https://github.com/auths-dev/auths) for commit signing (`brew install auths-dev/auths-cli/auths`)
