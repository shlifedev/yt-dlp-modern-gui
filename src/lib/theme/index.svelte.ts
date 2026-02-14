import { themes, type ThemeId } from "./themes"

let currentTheme = $state<ThemeId>("dark")

export function getTheme(): ThemeId {
  return currentTheme
}

export function setTheme(theme: ThemeId) {
  currentTheme = theme
  if (typeof document !== "undefined") {
    document.documentElement.setAttribute("data-theme", theme)
  }
}

export function initTheme(savedTheme?: string | null) {
  const theme = (savedTheme && savedTheme in themes ? savedTheme : "dark") as ThemeId
  currentTheme = theme
  if (typeof document !== "undefined") {
    document.documentElement.setAttribute("data-theme", theme)
  }
}
