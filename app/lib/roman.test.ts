import { describe, expect, it } from "vitest";
import { toRoman } from "./roman.ts";

describe("toRoman", () => {
  it("returns ○ for zero", () => {
    expect(toRoman(0)).toBe("○");
  });

  describe("additive sequences", () => {
    it("converts 1 to i", () => expect(toRoman(1)).toBe("i"));
    it("converts 2 to ii", () => expect(toRoman(2)).toBe("ii"));
    it("converts 3 to iii", () => expect(toRoman(3)).toBe("iii"));
    it("converts 8 to viii", () => expect(toRoman(8)).toBe("viii"));
  });

  describe("subtractive pairs", () => {
    it("converts 4 to iv", () => expect(toRoman(4)).toBe("iv"));
    it("converts 9 to ix", () => expect(toRoman(9)).toBe("ix"));
    it("converts 40 to xl", () => expect(toRoman(40)).toBe("xl"));
    it("converts 90 to xc", () => expect(toRoman(90)).toBe("xc"));
    it("converts 400 to cd", () => expect(toRoman(400)).toBe("cd"));
    it("converts 900 to cm", () => expect(toRoman(900)).toBe("cm"));
  });

  describe("clock boundary values", () => {
    it("converts 12 to xii", () => expect(toRoman(12)).toBe("xii"));
    it("converts 23 to xxiii", () => expect(toRoman(23)).toBe("xxiii"));
    it("converts 59 to lix", () => expect(toRoman(59)).toBe("lix"));
  });

  describe("date values", () => {
    it("converts 31 to xxxi", () => expect(toRoman(31)).toBe("xxxi"));
    it("converts 2026 to mmxxvi", () => expect(toRoman(2026)).toBe("mmxxvi"));
    it("converts 3999 to mmmcmxcix", () =>
      expect(toRoman(3999)).toBe("mmmcmxcix"));
  });

  it("throws for 4000", () => {
    expect(() => toRoman(4000)).toThrow();
  });
});
