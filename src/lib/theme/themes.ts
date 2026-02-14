export type ThemeId = "dark" | "red" | "light"

export interface ThemeColors {
  primary: string
  bg: string
  surface: string
  highlight: string
}

export const themes: Record<ThemeId, ThemeColors> = {
  dark: { primary: "#3b9eff", bg: "#0b0b10", surface: "#141419", highlight: "#1c1c24" },
  red: { primary: "#ef4444", bg: "#100a0a", surface: "#1a1214", highlight: "#241c1e" },
  light: { primary: "#2563eb", bg: "#f5f5f5", surface: "#ffffff", highlight: "#f3f4f6" },
}

export const themeList: { id: ThemeId; labelKey: string }[] = [
  { id: "dark", labelKey: "theme.dark" },
  { id: "red", labelKey: "theme.red" },
  { id: "light", labelKey: "theme.light" },
]
