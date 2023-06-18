import { redirect } from "@sveltejs/kit";

import type { PageServerLoad } from "./$types";

export const load = (({ url }) => {
  const version =
    url.searchParams.get("v") ?? url.searchParams.get("version") ?? "1.1.0";

  // eslint-disable-next-line @typescript-eslint/no-throw-literal, no-magic-numbers
  throw redirect(301, `/manual/tikz-feynman/${version}.pdf`);
}) satisfies PageServerLoad;
