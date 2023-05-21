import { redirect } from "@sveltejs/kit";

/**
 * Load function that throws a redirect error with status 301 and redirects to the root path.
 *
 * @throws {Error} If the function is called.
 *
 * @returns {void}
 */
export function load(): void {
  // eslint-disable-next-line no-magic-numbers, @typescript-eslint/no-throw-literal
  throw redirect(301, "/");
}
