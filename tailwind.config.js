/** @type {import('tailwindcss').Config} */
export default {
  content: ['./static/*.svg','./src/**/*.{js,ts,svelte}', './src/app.html'],
  theme: {
    extend: {},
  },
  plugins: [],
  corePlugins: {
    preflight: true,
  }
}

