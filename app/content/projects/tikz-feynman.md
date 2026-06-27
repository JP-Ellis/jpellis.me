---
github: JP-Ellis/tikz-feynman
slug: tikz-feynman
tagline: Feynman diagrams with human-readable code
title: TikZ-Feynman
---

Ti*k*Z-Feynman is a LaTeX package for typesetting Feynman diagrams
directly in LaTeX, without external tools. It leverages Ti*k*Z's graph
placement algorithms to automate vertex layout, while still allowing
fine-grained manual control for complex diagrams.

The package is available through [CTAN](https://ctan.org/pkg/tikz-feynman)
and ships with thorough [documentation](https://arxiv.org/pdf/1601.05437)
including a tutorial and an extensive example gallery.

If you use Ti*k*Z-Feynman in academic work, please cite:

> Ellis, Joshua P. "Ti*k*Z-Feynman: Feynman diagrams with Ti*k*Z."
> _Computer Physics Communications_ 210 (2017): 103–123.
> [doi:10.1016/j.cpc.2016.08.019](https://doi.org/10.1016/j.cpc.2016.08.019) ·
> [arXiv:1601.05437](https://arxiv.org/abs/1601.05437)

## Examples

A simple QED scattering diagram of two electrons exchanging a photon:

<img src="/projects/tikz-feynman/qed.png"
     alt="Two electrons scattering via photon exchange"
     class="invert-dark"
     style="display: block; margin: 0 auto;"
     width="300" />

```latex
\feynmandiagram [horizontal=a to b] {
  i1 -- [fermion] a -- [fermion] i2,
  a -- [photon] b,
  f1 -- [fermion] b -- [fermion] f2,
};
```

A "penguin" diagram with explicit momentum arrows:

<img src="/projects/tikz-feynman/penguin.png"
     alt="Penguin diagram with momentum arrows"
     class="invert-dark"
     style="display: block; margin: 0 auto;"
     width="200" />

```latex
\feynmandiagram [large, vertical=e to f] {
  a -- [fermion] b -- [photon, momentum=\(k\)] c -- [fermion] d,
  b -- [fermion, momentum'=\(p_1\)] e -- [fermion, momentum'=\(p_2\)] c,
  e -- [gluon] f,
  h -- [fermion] f -- [fermion] i;
};
```

A B-meson decay to π⁺π⁻ using the manual `diagram*` environment for
full positional control:

<img src="/projects/tikz-feynman/mixing.png"
     alt="B meson decaying to π⁺π⁻"
     class="invert-dark"
     style="display: block; margin: 0 auto;"
     width="500" />

```latex
\begin{tikzpicture}
  \begin{feynman}
    \vertex (a1) {\(\overline b\)};
    \vertex[right=1cm of a1] (a2);
    \vertex[right=1cm of a2] (a3);
    \vertex[right=1cm of a3] (a4) {\(b\)};
    % ... (see documentation for full example)
    \diagram* {
      {[edges=fermion] (a1) -- (a2) -- (a3) -- (a4)},
      (a2) -- [boson, edge label=\(W\)] (a3),
    };
  \end{feynman}
\end{tikzpicture}
```
