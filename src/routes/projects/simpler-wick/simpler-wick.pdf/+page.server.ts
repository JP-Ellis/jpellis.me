import { redirect } from "@sveltejs/kit";

import type { PageServerLoad } from "./$types";

export const load = (({ url }) => {
  const version =
    url.searchParams.get("v") ?? url.searchParams.get("version") ?? "latest";

  // eslint-disable-next-line @typescript-eslint/no-throw-literal, no-magic-numbers
  throw redirect(301, `/manual/simpler-wick/${version}.pdf`);
}) satisfies PageServerLoad;
