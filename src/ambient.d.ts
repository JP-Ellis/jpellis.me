// Squelch warnings of image imports from your assets dir
declare module "$lib/assets/images/*" {
  const meta: object[];
  export default meta;
}
