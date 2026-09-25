---
activity:
  recentCommits: false
github: JP-Ellis/simpler-wick
slug: simpler-wick
tagline: Wick contractions for quantum field theory, made readable
title: simpler-wick
---

Wick contractions appear in quantum field theory calculations where they connect field operators in time-ordered products to recover propagators. Writing them in LaTeX is tedious with the standard `simplewick` package, which requires manual bracket matching and is sensitive to spacing.

`simpler-wick` provides a cleaner syntax: contractions are specified by pairing labels, and the package handles the bracket drawing automatically.

## Usage

```latex
\usepackage{simpler-wick}

% Contract fields 1–4 and 2–3:
\(
  \wick{
    \c1\psi \c2\bar\psi \c2\phi \c1\bar\phi
  }
\)
```

Each contraction appears as a bracket above the expression. The bracket's height grows with the number after `\c`, so overlapping contractions sit at different levels.

Available on [CTAN](https://ctan.org/pkg/simpler-wick).
