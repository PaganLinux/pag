/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        bg: '#0a0a0f',
        'bg-card': '#1a1a2e',
        accent: '#7c3aed',
        'accent-hover': '#6d28d9',
        border: '#2a2a3e',
      },
    },
  },
  plugins: [],
};
