/** Site identity shared by the page head, feeds and structured data. */

export const SITE_NAME = "Joshua Ellis";

export const SITE_DESCRIPTION =
  "Joshua Ellis is a software engineer working on contract testing and open source in Rust and Python, and a former particle physicist.";

/** Address for the contact page, the footer and structured data. */
export const CONTACT_EMAIL = "website@jpellis.me";

/** Image for link previews, relative to the site root. */
export const SOCIAL_IMAGE = "/android-chrome-512x512.png";

/** Profiles that identify the site owner elsewhere. */
export const PROFILES = [
  "https://github.com/JP-Ellis",
  "https://linkedin.com/in/joshuapellis",
  "https://orcid.org/0000-0003-2556-1536",
] as const;

const HTML_SUFFIX_RE = /\.html$/u;
const INDEX_SUFFIX_RE = /\/index$/u;

/**
 * Maps a request path to the page's public path. With `build.format: "file"`
 * a prerendered page sees its output file (`/contact.html`) as its path; the
 * site serves it without the extension.
 */
export function publicPath(pathname: string): string {
  const path = pathname
    .replace(HTML_SUFFIX_RE, "")
    .replace(INDEX_SUFFIX_RE, "/");
  return path === "" ? "/" : path;
}

/** Formats a page title as `Page · Joshua Ellis`, or the bare name. */
export function pageTitle(page?: string): string {
  return page ? `${page} · ${SITE_NAME}` : SITE_NAME;
}

/** schema.org Person describing the site owner. */
export function personJsonLd(site: URL): Record<string, unknown> {
  return {
    "@context": "https://schema.org",
    "@type": "Person",
    name: SITE_NAME,
    alternateName: ["Joshua P. Ellis", "JP-Ellis"],
    url: site.href,
    email: `mailto:${CONTACT_EMAIL}`,
    jobTitle: "Software Engineer",
    sameAs: PROFILES,
  };
}
