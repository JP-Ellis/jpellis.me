+++
github = "JP-Ellis/simpler-wick"
slug = "simpler-wick"
tagline = "Wick contractions for quantum field theory, made readable"
title = "simpler-wick"

[activity]
recent_commits = false
+++

Wick contractions appear in quantum field theory calculations where they connect field operators in time-ordered products to recover propagators. Writing them in LaTeX is tedious with the standard `simplewick` package, which requires manual bracket matching and is sensitive to spacing.

`simpler-wick` provides a cleaner syntax: contractions are specified by pairing labels, and the package handles the bracket drawing automatically.

## Usage

```latex
\usepackage{simpler-wick}

% Contract fields 1–3 and 2–4:
\(
  \wick{
    \c1\psi \c2\bar\psi \c2\phi \c1\bar\phi
  }
\)
```

The package renders arcs above or below the expression at the correct height, avoids overlapping brackets, and handles nested contractions cleanly. It works seamlessly with `amsmath` environments.

Available on [CTAN](https://ctan.org/pkg/simpler-wick).
