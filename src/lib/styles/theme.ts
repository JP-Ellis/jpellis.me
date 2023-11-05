/* eslint-disable line-comment-position */

/* eslint-disable no-inline-comments */
import type { CustomThemeConfig } from "@skeletonlabs/tw-plugin";

export const jpellisTheme: CustomThemeConfig = {
  name: "jpellis-theme",
  properties: {
    // =~= Theme Properties =~=
    "--theme-font-family-base": `system-ui`,
    "--theme-font-family-heading": `system-ui`,
    "--theme-font-color-base": "0 0 0",
    "--theme-font-color-dark": "255 255 255",
    "--theme-rounded-base": "9999px",
    "--theme-rounded-container": "8px",
    "--theme-border-base": "1px",
    // =~= Theme On-X Colors =~=
    "--on-primary": "255 255 255",
    "--on-secondary": "0 0 0",
    "--on-tertiary": "255 255 255",
    "--on-success": "0 0 0",
    "--on-warning": "0 0 0",
    "--on-error": "255 255 255",
    "--on-surface": "255 255 255",
    // =~= Theme Colors  =~=
    // primary | #d43400
    "--color-primary-50": "249 225 217", // #f9e1d9
    "--color-primary-100": "246 214 204", // #f6d6cc
    "--color-primary-200": "244 204 191", // #f4ccbf
    "--color-primary-300": "238 174 153", // #eeae99
    "--color-primary-400": "225 113 77", // #e1714d
    "--color-primary-500": "212 52 0", // #d43400
    "--color-primary-600": "191 47 0", // #bf2f00
    "--color-primary-700": "159 39 0", // #9f2700
    "--color-primary-800": "127 31 0", // #7f1f00
    "--color-primary-900": "104 25 0", // #681900
    // secondary | #20b797
    "--color-secondary-50": "222 244 239", // #def4ef
    "--color-secondary-100": "210 241 234", // #d2f1ea
    "--color-secondary-200": "199 237 229", // #c7ede5
    "--color-secondary-300": "166 226 213", // #a6e2d5
    "--color-secondary-400": "99 205 182", // #63cdb6
    "--color-secondary-500": "32 183 151", // #20b797
    "--color-secondary-600": "29 165 136", // #1da588
    "--color-secondary-700": "24 137 113", // #188971
    "--color-secondary-800": "19 110 91", // #136e5b
    "--color-secondary-900": "16 90 74", // #105a4a
    // tertiary | #3273b7
    "--color-tertiary-50": "224 234 244", // #e0eaf4
    "--color-tertiary-100": "214 227 241", // #d6e3f1
    "--color-tertiary-200": "204 220 237", // #ccdced
    "--color-tertiary-300": "173 199 226", // #adc7e2
    "--color-tertiary-400": "112 157 205", // #709dcd
    "--color-tertiary-500": "50 115 183", // #3273b7
    "--color-tertiary-600": "45 104 165", // #2d68a5
    "--color-tertiary-700": "38 86 137", // #265689
    "--color-tertiary-800": "30 69 110", // #1e456e
    "--color-tertiary-900": "25 56 90", // #19385a
    // success | #84cc16
    "--color-success-50": "237 247 220", // #edf7dc
    "--color-success-100": "230 245 208", // #e6f5d0
    "--color-success-200": "224 242 197", // #e0f2c5
    "--color-success-300": "206 235 162", // #ceeba2
    "--color-success-400": "169 219 92", // #a9db5c
    "--color-success-500": "132 204 22", // #84cc16
    "--color-success-600": "119 184 20", // #77b814
    "--color-success-700": "99 153 17", // #639911
    "--color-success-800": "79 122 13", // #4f7a0d
    "--color-success-900": "65 100 11", // #41640b
    // warning | #ef9103
    "--color-warning-50": "253 239 217", // #fdefd9
    "--color-warning-100": "252 233 205", // #fce9cd
    "--color-warning-200": "251 228 192", // #fbe4c0
    "--color-warning-300": "249 211 154", // #f9d39a
    "--color-warning-400": "244 178 79", // #f4b24f
    "--color-warning-500": "239 145 3", // #ef9103
    "--color-warning-600": "215 131 3", // #d78303
    "--color-warning-700": "179 109 2", // #b36d02
    "--color-warning-800": "143 87 2", // #8f5702
    "--color-warning-900": "117 71 1", // #754701
    // error | #a01417
    "--color-error-50": "241 220 220", // #f1dcdc
    "--color-error-100": "236 208 209", // #ecd0d1
    "--color-error-200": "231 196 197", // #e7c4c5
    "--color-error-300": "217 161 162", // #d9a1a2
    "--color-error-400": "189 91 93", // #bd5b5d
    "--color-error-500": "160 20 23", // #a01417
    "--color-error-600": "144 18 21", // #901215
    "--color-error-700": "120 15 17", // #780f11
    "--color-error-800": "96 12 14", // #600c0e
    "--color-error-900": "78 10 11", // #4e0a0b
    // surface | #13151b
    "--color-surface-50": "220 220 221", // #dcdcdd
    "--color-surface-100": "208 208 209", // #d0d0d1
    "--color-surface-200": "196 197 198", // #c4c5c6
    "--color-surface-300": "161 161 164", // #a1a1a4
    "--color-surface-400": "90 91 95", // #5a5b5f
    "--color-surface-500": "19 21 27", // #13151b
    "--color-surface-600": "17 19 24", // #111318
    "--color-surface-700": "14 16 20", // #0e1014
    "--color-surface-800": "11 13 16", // #0b0d10
    "--color-surface-900": "9 10 13", // #090a0d
  },
};
