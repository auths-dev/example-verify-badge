# Verification Badges with Auths

[![Verify Commits](https://github.com/auths-dev/example-verify-badge/actions/workflows/verify-commits.yml/badge.svg)](https://github.com/auths-dev/example-verify-badge/actions/workflows/verify-commits.yml?query=branch%3Amain+event%3Apush)

Two ways to show that commits in your repo are cryptographically verified by [Auths](https://github.com/auths-dev/auths).

## 1. GitHub Action Badge

A CI workflow verifies commit signatures on every push and PR. The badge above reflects the latest result — green means all commits are signed by authorized keys.

**Setup:** add [`.github/workflows/verify-commits.yml`](.github/workflows/verify-commits.yml) and an [`.auths/allowed_signers`](.auths/allowed_signers) file to your repo. See the [auths-verify-github-action](https://github.com/marketplace/actions/verify-commit-signatures-with-auths) docs for configuration options.

**Embed in your README:**

```markdown
[![Verify Commits](https://github.com/YOUR-ORG/YOUR-REPO/actions/workflows/verify-commits.yml/badge.svg)](https://github.com/YOUR-ORG/YOUR-REPO/actions/workflows/verify-commits.yml?query=branch%3Amain+event%3Apush)
```

## 2. Custom Widget Badge

The `<auths-verify>` web component performs cryptographic verification entirely in the browser using WASM — no backend required. It fetches the repo's signing data from `refs/auths/registry` and verifies Ed25519 signatures client-side.

**Live demo:** [auths-dev.github.io/example-verify-badge](https://auths-dev.github.io/example-verify-badge/)

**Add to any HTML page:**

```html
<script type="module" src="https://unpkg.com/@auths-dev/verify/dist/auths-verify.mjs"></script>

<!-- Compact badge -->
<auths-verify repo="your-org/your-repo" forge="github" mode="badge"></auths-verify>

<!-- Expanded details -->
<auths-verify repo="your-org/your-repo" forge="github" mode="detail"></auths-verify>

<!-- Hover tooltip -->
<auths-verify repo="your-org/your-repo" forge="github" mode="tooltip"></auths-verify>
```

Works on GitHub Pages, documentation sites, personal websites — anywhere you control the HTML. Does **not** render in GitHub READMEs (GitHub strips custom elements and scripts).
