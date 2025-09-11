/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.{rs,html,css}",
    "./public/**/*.html",
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
      },
      colors: {
        primary: {
          50: '#fef7f4',
          100: '#fdeee8',
          200: '#fad4c6',
          300: '#f6b9a4',
          400: '#ee8960',
          500: '#D34516', // Main accent color
          600: '#be3e14',
          700: '#9e3411',
          800: '#80290e',
          900: '#69210c',
        },
        neutral: {
          50: '#fafafa',
          100: '#f5f5f5', 
          200: '#e5e5e5',
          300: '#d4d4d4',
          400: '#a3a3a3',
          500: '#737373',
          600: '#525252',
          700: '#404040',
          800: '#3c3b43', // Main text color
          900: '#171717',
        },
      },
      animation: {
        'fade-in': 'fadeIn 0.5s ease-in-out',
        'slide-up': 'slideUp 0.3s ease-out',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideUp: {
          '0%': { transform: 'translateY(10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
      },
    },
  },
  plugins: [
    // Add any Tailwind plugins you need
  ],
}
