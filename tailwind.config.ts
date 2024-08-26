import daisyui from 'daisyui';
import type { Config } from 'tailwindcss';
import forms from '@tailwindcss/forms';
import typography from '@tailwindcss/typography';
import tailwindScrollbar from 'tailwind-scrollbar';

export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {}
	},

	plugins: [forms, typography, daisyui, tailwindScrollbar({ nocompatible: true })],

	daisyui: {
		themes: [
			{
				'tagstudio-theme': {
					primary: 'oklch(67.99% 0.1155931375154364 200.6506960142565)',
					secondary: 'oklch(43.2% 0.211 292.76)',
					accent: 'oklch(94.91% 0 0)',
					neutral: 'oklch(36.23% 0.015 259.81)',
					'base-100': 'oklch(27.49% 0.019 258.37)',
					info: 'oklch(48.82% 0.217 264.38)',
					success: 'oklch(59.6% 0.127 163.23)',
					warning: 'oklch(76.86% 0.165 70.08)',
					error: 'oklch(57.71% 0.215 27.33)'
				}
			}
		]
	}
} satisfies Config;
