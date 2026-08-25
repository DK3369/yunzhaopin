import tseslint from 'typescript-eslint'
import { rules as nuxtLayerRules } from 'eslint-plugin-nuxt-layers'

const layerBoundaries = [
  'error',
  {
    root: 'layers',
    aliases: ['#layers'],
    layers: {
      base: [],
      ui: ['base'],
    },
  },
]

export default tseslint.config(
  {
    ignores: ['**/node_modules/**', '**/.nuxt/**', '**/.output/**', '**/dist/**', '**/*.d.ts'],
  },
  {
    files: ['**/*.{js,mjs,ts}'],
    languageOptions: {
      parser: tseslint.parser,
    },
    plugins: {
      'nuxt-layers': { rules: nuxtLayerRules },
    },
    rules: {
      'nuxt-layers/layer-boundaries': layerBoundaries,
    },
  },
)
