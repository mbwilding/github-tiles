# GitHub Tiles

Generate SVG tiles for your GitHub profile README.

## Usage

```yaml
name: GitHub Tiles

on:
  schedule:
    - cron: "0 0 * * *" # Daily
  workflow_dispatch:

jobs:
  github-tiles:
    runs-on: ubuntu-latest
    permissions:
      contents: write
    steps:
      - name: GitHub Tiles
        uses: mbwilding/github-tiles@v1
        with:
          token: ${{ secrets.TOKEN }}
```

## Setup

1. [Create a personal access token](https://github.com/settings/tokens/new?scopes=repo,read:user&description=GitHub%20Tiles) with `repo` and `read:user` scopes
2. Set expiration to "No expiration"
3. Add the token as a repository secret named `TOKEN`
4. Create the workflow file at `.github/workflows/github-tiles.yml`

## Inputs

| Input | Description | Required | Default |
|-------|-------------|----------|---------|
| `token` | GitHub personal access token | Yes | - |
| `output` | Output directory for SVGs | No | `assets` |
| `tiles` | Tiles to generate (comma-separated) | No | `statistics,languages,contributions` |
| `private` | Include private repositories | No | `false` |
| `forks` | Include forked repositories | No | `false` |
| `languages-limit` | Number of languages to display | No | `5` |
| `contributions-limit` | Number of contributions to display | No | `10` |
| `contributions-min-stars` | Minimum stars for contributions | No | `0` |
| `opaque` | Render with opaque background | No | `false` |
| `optimize` | Optimize SVG output | No | `true` |
| `log-level` | Log level (error, warn, info, debug, trace) | No | `info` |

## Tiles

### Statistics

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./assets/statistics_dark.svg" />
  <source media="(prefers-color-scheme: light)" srcset="./assets/statistics_light.svg" />
  <img src="./assets/statistics_dark.svg" />
</picture>

### Languages

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./assets/languages_dark.svg" />
  <source media="(prefers-color-scheme: light)" srcset="./assets/languages_light.svg" />
  <img src="./assets/languages_dark.svg" />
</picture>

### Contributions

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./assets/contributions_dark.svg" />
  <source media="(prefers-color-scheme: light)" srcset="./assets/contributions_light.svg" />
  <img src="./assets/contributions_dark.svg" />
</picture>

## Displaying in your README

Use the `<picture>` element to support light and dark themes:

```html
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./assets/statistics_dark.svg" />
  <source media="(prefers-color-scheme: light)" srcset="./assets/statistics_light.svg" />
  <img src="./assets/statistics_dark.svg" />
</picture>
```

## License

MIT
