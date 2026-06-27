export type Status = "active" | "maintained" | "archived" | "shipped" | "wip";

export type ProjectLink =
  | { kind: "github"; slug: string }
  | { kind: "external"; url: string };

export interface ProjectEntry {
  name: string;
  kind: string;
  stack: string;
  summary: string;
  status: Status;
  link: ProjectLink | null;
}

export const PROJECTS: ProjectEntry[] = [
  {
    name: "pact-python",
    kind: "OSS · library",
    stack: "rust · python · ffi",
    summary:
      "Python bindings for Pact, rebuilt over a Rust FFI core. The version most pact-python users actually reach for.",
    status: "active",
    link: { kind: "github", slug: "pact-foundation/pact-python" },
  },
  {
    name: "tikz-feynman",
    kind: "OSS · LaTeX",
    stack: "tex · tikz",
    summary:
      "A LaTeX package for typesetting Feynman diagrams with TikZ; adopted across particle physics with 400+ academic citations.",
    status: "maintained",
    link: { kind: "github", slug: "JP-Ellis/tikz-feynman" },
  },
  {
    name: "rust-skiplist",
    kind: "OSS · library",
    stack: "rust",
    summary: "Skiplist data structure implementation in Rust.",
    status: "maintained",
    link: { kind: "github", slug: "JP-Ellis/rust-skiplist" },
  },
  {
    name: "mathematica-notebook-filter",
    kind: "OSS · tool",
    stack: "rust",
    summary: "Filters Mathematica notebooks for clean version-control diffs.",
    status: "maintained",
    link: { kind: "github", slug: "JP-Ellis/mathematica-notebook-filter" },
  },
  {
    name: "simpler-wick",
    kind: "OSS · LaTeX",
    stack: "tex · tikz",
    summary: "LaTeX macros for Wick contractions in quantum field theory.",
    status: "maintained",
    link: { kind: "github", slug: "JP-Ellis/simpler-wick" },
  },
  {
    name: "boltzmann-solver",
    kind: "research · numerics",
    stack: "rust",
    summary:
      "Solver for highly-coupled Boltzmann equations with rates spanning many decades; custom quadrature.",
    status: "archived",
    link: { kind: "github", slug: "hep-rs/boltzmann-solver" },
  },
  {
    name: "dotfiles",
    kind: "personal · glue",
    stack: "nix · shell",
    summary:
      "Opinionated dotfiles: nix flake, fish prompt, helix config, backup scripts.",
    status: "maintained",
    link: { kind: "github", slug: "JP-Ellis/dotfiles" },
  },
  {
    name: "jpellis.me",
    kind: "personal · site",
    stack: "typescript · astro",
    summary: "This site, rebuilt in Astro on Cloudflare Workers.",
    status: "maintained",
    link: { kind: "github", slug: "JP-Ellis/jpellis.me" },
  },
  {
    name: "borrow-checker",
    kind: "personal · app",
    stack: "rust",
    summary:
      "Personal finance app for tracking accounts, spending, and cashflow — named for the compiler feature that stops you spending twice.",
    status: "wip",
    link: { kind: "github", slug: "JP-Ellis/borrow-checker" },
  },
  {
    name: "amber-api",
    kind: "personal · library",
    stack: "rust",
    summary: "Rust client for Amber Electric's real-time energy pricing API.",
    status: "maintained",
    link: { kind: "github", slug: "JP-Ellis/amber-api" },
  },
  {
    name: "enphase-api",
    kind: "personal · library",
    stack: "rust",
    summary: "Rust client for the Enphase solar monitoring API.",
    status: "maintained",
    link: { kind: "github", slug: "JP-Ellis/enphase-api" },
  },
  {
    name: "repo-manage",
    kind: "personal · tool",
    stack: "python",
    summary:
      "Convenience CLI for applying consistent settings and CI workflows across a fleet of personal repositories.",
    status: "maintained",
    link: { kind: "github", slug: "JP-Ellis/repo-manage" },
  },
  {
    name: "azure data accelerator",
    kind: "consulting · internal",
    stack: "azure · bicep · python",
    summary:
      "Modular, configuration-driven accelerator for deploying standardised Azure data-platform infrastructure; used as an internal KPMG template.",
    status: "shipped",
    link: null,
  },
  {
    name: "pactflow-ai",
    kind: "work · internal",
    stack: "rust · python · ai",
    summary:
      "AI-powered platform that automates the creation and maintenance of contract tests, integrating into existing development tools to save teams up to 60% of manual testing time.",
    status: "active",
    link: { kind: "external", url: "https://pactflow.io/ai/" },
  },
];
