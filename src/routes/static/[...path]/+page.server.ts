import { redirect } from "@sveltejs/kit";

import type { PageServerLoad } from "./$types";

export const load = (({ params }) => {
  // eslint-disable-next-line @typescript-eslint/no-throw-literal, no-magic-numbers
  redirect(301, `/${params.path}`);
}) satisfies PageServerLoad;
