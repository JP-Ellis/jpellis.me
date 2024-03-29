// Squelch warnings of image imports from your assets dir
declare module "$lib/assets/*.png?as=run" {
  const meta: object[];
  export default meta;
}
declare module "$lib/assets/*.png?as=run:0" {
  const meta: object[];
  export default meta;
}
declare module "$lib/assets/*.jpg?as=run" {
  const meta: object[];
  export default meta;
}
declare module "$lib/assets/*.jpg?as=run:0" {
  const meta: object[];
  export default meta;
}

declare module "$lib/assets/*.svg?component" {
  import type { ComponentType, SvelteComponentTyped } from "svelte";
  import type { SVGAttributes } from "svelte/elements";

  const content: ComponentType<
    SvelteComponentTyped<SVGAttributes<SVGSVGElement>>
  >;

  export default content;
}

declare module "*.svg?src" {
  const content: string;
  export default content;
}

declare module "*.svg?url" {
  const content: string;
  export default content;
}

declare module "*.svg?dataurl" {
  const content: string;
  export default content;
}
