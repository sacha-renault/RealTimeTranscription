import { defineStore } from 'pinia'

export const useThemeStore = defineStore('theme', {
    state: () => ({
        isDarkTheme: false, // Default theme
    }),
    actions: {
        setDarkTheme(isDarkTheme: boolean) {
            this.isDarkTheme = isDarkTheme;
        }
    },
})