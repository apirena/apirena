# Onboarding: shadcn-svelte Design System Setup for reqsmith Desktop

Owner: GitHub Copilot
Date: 2025-08-07
Task ID: shadcn-svelte-setup
Scope: apps/desktop (SvelteKit + Tauri)

---

## 1) Task Summary
Integrate the shadcn-svelte component library and Tailwind CSS into the SvelteKit desktop app so that all UI uses a consistent design system going forward. Establish the tooling, configuration, and initial component(s), and plan a migration path for existing views.

High-level deliverables:
- TailwindCSS configured for SvelteKit app (apps/desktop).
- shadcn-svelte initialized with project aliases and global CSS.
- Example component (Button) added and rendered to verify setup.
- Migration guidance and checklist to transition existing components to the design system.


## 2) Repository + App Overview (what exists today)
- Monorepo managed by pnpm workspaces.
- Desktop app: SvelteKit (adapter-static SPA) + Tauri.
  - Location: apps/desktop
  - SvelteKit ^2.9, Svelte ^5, Vite ^6
  - No Tailwind yet; global styles live in `src/app.css` with a custom CSS variable palette.
  - `+layout.svelte` imports `../app.css`.
  - Aliases: default `$lib` (no custom aliases in `svelte.config.js`).
- Rust libs and other packages present; not relevant to frontend styling setup.

Key files checked:
- apps/desktop/svelte.config.js (adapter-static SPA; no aliases set)
- apps/desktop/package.json (no tailwind/postcss dependencies)
- apps/desktop/src/app.css (custom CSS variables + resets)

Implication: We will add Tailwind and shadcn-svelte inside apps/desktop without affecting other packages.


## 3) Constraints, Risks, Assumptions
- Tauri SPA (SSR disabled) is fine; shadcn-svelte works client-side.
- Svelte 5 compatibility: use latest shadcn-svelte which supports Svelte 5.
- Tailwind version: use whatever the Svelte CLI (“sv add tailwindcss”) installs for SvelteKit 2 + Vite 6. If Tailwind v4 is default, ensure shadcn-svelte template matches; otherwise pin Tailwind v3.x.
- Global CSS overwrite: the shadcn init will overwrite `src/app.css`. We must preserve existing CSS variables and optionally fold them into Tailwind or keep them alongside Tailwind layers.
- Aliases: Using defaults (`$lib`, `$lib/components`, `$lib/components/ui`, `$lib/utils`, `$lib/hooks`).


## 4) Proposed Plan (step-by-step)
1. Add TailwindCSS to apps/desktop using the Svelte CLI.
   - In apps/desktop: run "pnpm dlx sv add tailwindcss".
   - Keep global stylesheet path as `src/app.css`. Merge existing CSS variables into the generated Tailwind layers (e.g., put variables under `@layer base`), or append the existing file contents below Tailwind’s directives.

2. Initialize shadcn-svelte.
   - In apps/desktop: run "pnpm dlx shadcn-svelte@latest init".
   - Prompts (preferred answers):
     - Base color: Slate
     - Global CSS file: src/app.css
     - Import alias for lib: $lib
     - Import alias for components: $lib/components
     - Import alias for utils: $lib/utils
     - Import alias for hooks: $lib/hooks
     - Import alias for ui: $lib/components/ui

3. Add a first component to validate install.
   - In apps/desktop: run "pnpm dlx shadcn-svelte@latest add button".
   - Render it on the home page to verify styling and tree-shaking work.

4. Theming decisions.
   - Short term: Use shadcn defaults (Slate). Keep existing CSS variables for now to avoid breaking current views during migration.
   - Medium term: Map our existing design tokens to Tailwind theme and/or shadcn CSS variables. The goal is a single source of truth (Tailwind theme tokens) that power shadcn UI and custom components.

5. Migration strategy for existing components in `src/lib/components`.
   - Replace ad-hoc styles with Tailwind utility classes and shadcn primitives.
   - Start with interactive controls (Button, Input, Select, Tabs, Dialog, Tooltip, Toast).
   - Standardize layout primitives (Container, Card, Separator, ScrollArea) to reduce custom CSS.
   - Establish a UI guidelines doc for patterns (forms, dialogs, validation, empty states, error states).

6. Developer experience.
   - Ensure `dev`, `build`, and Tauri workflows still work: "pnpm dev" inside apps/desktop; Tauri via the existing script.
   - Add a minimal UI showcase route or page section to preview common components during migration.


## 5) Detailed Execution Notes
- Tailwind setup (SvelteKit): the CLI will add Tailwind and PostCSS configs and update `app.css` to include `@tailwind base; @tailwind components; @tailwind utilities;`.
- Preserve existing CSS variables from `app.css`. Place them either:
  - Above Tailwind layers so utilities can use them, or
  - Inside `@layer base { :root { ... } }` to keep the cascade predictable; include dark-mode tokens similarly.
- shadcn-svelte generates files into `$lib/components/ui/...` and may add `components.json` at the project root (apps/desktop). Commit those.
- Verify type-checks still pass (`pnpm run check` in apps/desktop).
- If Tailwind v4 is installed and shadcn templates expect v3 config shape, pin Tailwind to a compatible version in apps/desktop and re-run the init. Otherwise proceed with v4-compatible shadcn templates.


## 6) Success Criteria / Acceptance
- Tailwind and shadcn-svelte are installed and configured within apps/desktop.
- The Button component is added and renders correctly on `+page.svelte` or `+layout.svelte`.
- No runtime errors in Vite dev or Tauri dev modes.
- Clear documented migration path and checklist for converting existing components.


## 7) Rollout Checklist
- [ ] Add Tailwind to apps/desktop.
- [ ] Preserve and merge existing `app.css` variables with Tailwind layers.
- [ ] Run shadcn-svelte init with agreed aliases.
- [ ] Add `button` and verify in the UI.
- [ ] Commit generated `components.json`, tailwind/postcss configs, and any created UI files.
- [ ] Create a `ui-demo` area (optional) for showcasing components during migration.
- [ ] Convert at least one existing component to shadcn primitives as a sample.


## 8) Open Questions for the Maintainer
1. Scope confirmation: Is the design system required for the desktop app only, or should we also plan for other frontends (if any)?
2. Branding: Should we keep the current color palette or fully switch to shadcn defaults (Slate) and later remap to brand colors?
3. Typography: Any specific font choices beyond system UI fonts? Should we introduce a font via Tailwind theme?
4. Dark mode: Current CSS variables include dark mode via `prefers-color-scheme`. Do we want class-based dark mode (Tailwind `class` strategy) for explicit toggling?
5. Component priorities: Which components should be migrated first (buttons/inputs/dialogs/tabs/tables)?
6. Tailwind version policy: Are we okay with the latest Tailwind version the Svelte CLI installs, or do we want to pin to a specific major for stability?


## 9) Future Enhancements
- Build a small Storybook-like docs page (in-app) or a route that lists available shadcn components configured for the project.
- Create wrapper components for common patterns (e.g., PrimaryButton, DangerButton) if brand tokens diverge.
- Establish ESLint/Prettier rules for class ordering (e.g., `prettier-plugin-tailwindcss`).


## 10) Quick Reference (commands to be run in apps/desktop)
- Install Tailwind: "pnpm dlx sv add tailwindcss"
- Init shadcn-svelte: "pnpm dlx shadcn-svelte@latest init"
- Add a component: "pnpm dlx shadcn-svelte@latest add button"
- Dev server: "pnpm run dev"
- Type check: "pnpm run check"
- Tauri dev: "pnpm run tauri dev"


## 11) Current Findings Snapshot
- No Tailwind or shadcn currently present in apps/desktop.
- `src/app.css` contains custom tokens that we should retain during the transition.
- Default `$lib` alias is in use; no alias changes needed.


## 12) Risks & Mitigations
- Overwrite of `app.css`: back up and re-merge content; keep variables under `@layer base`.
- Version mismatch (Tailwind/shadcn templates): be ready to pin Tailwind or update templates.
- Visual regressions: plan incremental rollout; add a UI demo route to validate components.


## 13) Time Estimate
- Initial setup (Tailwind + shadcn + sample button): 1–2 hours.
- Token mapping and theming: 1–3 hours.
- Migrating initial set of components: 0.5–1 day depending on scope.


## 14) Links
- shadcn-svelte docs: https://www.shadcn-svelte.com/
- SvelteKit + Tailwind guide: https://svelte.dev/docs/kit/adding-integrations#tailwind-css
- Tauri + SvelteKit: https://v2.tauri.app/start/frontend/sveltekit/

---

## Status Update — 2025-08-07
- TailwindCSS added to apps/desktop via Svelte CLI (Tailwind v4). Forms + Typography plugins enabled.
- app.css preserved and merged (backup saved at `apps/desktop/src/app.css.backup-pre-tailwind-2025-08-07.css`).
- Vite config consolidated in `apps/desktop/vite.config.ts` with both `@sveltejs/kit/vite` and `@tailwindcss/vite` plugins and Tauri dev server settings.
  - Note: `apps/desktop/vite.config.js` still exists from before; prefer using the TS config going forward to avoid ambiguity.
- Blocker: shadcn-svelte init requires Node >= 20. Current Node is 18.20.8.

### Next Actions
1) Upgrade Node to >= 20 (recommend nvm):
   - Install nvm (if not installed): https://github.com/nvm-sh/nvm
   - nvm install 20 && nvm use 20
   - corepack enable (optional, for pnpm)
2) Initialize shadcn-svelte (from apps/desktop):
   - pnpm dlx shadcn-svelte@latest init --base-color slate --css src/app.css --lib-alias $lib --components-alias $lib/components --utils-alias $lib/utils --hooks-alias $lib/hooks --ui-alias $lib/components/ui -o
3) Add first component and verify:
   - pnpm dlx shadcn-svelte@latest add button
   - Import Button on `src/routes/+page.svelte` and render for smoke test.
4) Optional cleanup: remove legacy `vite.config.js` to avoid confusion.
