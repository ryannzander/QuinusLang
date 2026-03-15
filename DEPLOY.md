# Docs deployment

Docs are built with **mdBook** and deployed to the `gh-pages` branch via GitHub Actions.

**Required:** Repo → Settings → Pages → Source: **Deploy from a branch** → Branch: `gh-pages` → `/ (root)`.

After pushing, the workflow builds the book and pushes to gh-pages. Site: https://ryannzander.github.io/QuinusLang/
