// biome-ignore-all lint/style/noMagicNumbers: the numeric literals are the Roman numeral definitions and the 0–3999 domain bounds
/** Additive/subtractive lookup table, highest value first. */
const TABLE: [number, string][] = [
  [1000, "m"],
  [900, "cm"],
  [500, "d"],
  [400, "cd"],
  [100, "c"],
  [90, "xc"],
  [50, "l"],
  [40, "xl"],
  [10, "x"],
  [9, "ix"],
  [5, "v"],
  [4, "iv"],
  [1, "i"],
];

/**
 * Convert a non-negative integer to a lowercase Roman numeral string.
 *
 * Returns `"○"` (U+25CB WHITE CIRCLE) for `0`.
 * Returns lowercase Roman numerals for `1–3999`.
 * Throws a `RangeError` for values ≥ 4000, negative numbers, or non-integers.
 */
export function toRoman(n: number): string {
  if (!Number.isInteger(n) || n < 0) {
    throw new RangeError(`toRoman: ${n} is not a non-negative integer`);
  }
  if (n === 0) {
    return "○";
  }
  if (n >= 4000) {
    throw new RangeError(`toRoman: ${n} is out of range (0–3999)`);
  }
  let result = "";
  let remaining = n;
  for (const [value, symbol] of TABLE) {
    while (remaining >= value) {
      result += symbol;
      remaining -= value;
    }
  }
  return result;
}
