import { defineStore } from 'pinia'

export const useThemeStore = defineStore('theme', {
    state: () => ({
        isDarkTheme: true, // Default theme
    }),
    actions: {
        setDarkTheme(isDarkTheme: boolean) {
            this.isDarkTheme = isDarkTheme;
        }
    },
})