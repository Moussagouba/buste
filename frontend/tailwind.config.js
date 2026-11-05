/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        dark: {
          900: '#0b0f19',
          850: '#111827',
          800: '#1e293b',
          700: '#334155',
          600: '#475569',
        },
        melbet: {
          yellow: '#f3a812',
          orange: '#ff8c00',
          dark: '#1c1e22',
        },
        success: {
          500: '#10b981',
          600: '#059669',
        },
        danger: {
          500: '#ef4444',
          600: '#dc2626',
        }
      }
    },
  },
  plugins: [],
}

