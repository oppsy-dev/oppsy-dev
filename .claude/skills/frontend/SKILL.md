---
name: frontend
description: Rules and guidelines how to structure frontend React based project, how to write components, which best practises to apply etc.
skills:
  - design-guide
  - solid
---

# Frontend Development Guidelines

This project uses **React 19**, **TypeScript**, **CSS Modules**, and **Yarn**.

---

## 1. Project Structure

```
src/
  components/ # Shared/reusable UI components
  pages/ # Page-level components (one per route)
  routes/ # Route definitions (Routes.tsx only)
  hooks/ # Custom React hooks
  context/ # React context definitions and providers
  stores/ # Zustand domain slices (one file per domain)
  utils/ # Pure utility functions
  styles/ # Global CSS (reset, typography, CSS variables)
```

**Every component lives in its own dedicated folder.**

If a component is only used by one specific parent component (and is not generic/reusable), nest it inside the parent's folder rather than placing it in the top-level `components/` or `pages/` directory:

```
src/
  pages/
    Dashboard/
      Dashboard.tsx
      Dashboard.module.css
      StatsPanel/              # only used by Dashboard
        StatsPanel.tsx
        StatsPanel.module.css
        StatCard/              # only used by StatsPanel
          StatCard.tsx
          StatCard.module.css
  components/
    Button/                    # generic, reusable — lives at top level
      Button.tsx
      Button.module.css
```

The rule: **if it's reusable across multiple places → `src/components/`; if it belongs to one parent → nest it inside that parent's folder.**

---

## 2. Component Patterns

- **Functional components only** — no class components.
- Use `.tsx` for components, `.ts` for hooks, utilities, and plain logic.
- One component per file; filename must match the component name (PascalCase).
- Define props with a named type directly above the component:

```tsx
type ButtonProps = {
  label: string;
  onClick: () => void;
  variant?: 'primary' | 'secondary';
};

export function Button({ label, onClick, variant = 'primary' }: ButtonProps) {
  ...
}
```

- Use **named exports** — no default exports. Import as `import { Button } from '../components/Button/Button'`.

- Extract logic into custom hooks (`src/hooks/`) when a component grows complex.
- Prefer composition over prop drilling beyond two levels.
- Keep components small and single-responsibility.

---

## 3. State Management

- **`useState`** — for local, UI-only state scoped to a single component.
- **[Zustand](https://github.com/pmndrs/zustand)** — for shared/global state across components. If local state grows complex, lift it into a Zustand store instead of reaching for `useReducer`.

Define stores in `src/stores/`, one file per domain slice. Use the TypeScript form `create<State>()(...)` (double invocation — required for correct TS inference):

```ts
// src/stores/useAuthStore.ts
import { create } from 'zustand';

interface AuthState {
  user: User | null;
  setUser: (user: User | null) => void;
}

export const useAuthStore = create<AuthState>()((set) => ({
  user: null,
  setUser: (user) => set({ user }),
}));
```

Use in components — select only the slice you need to avoid unnecessary re-renders:

```tsx
const user = useAuthStore((state) => state.user);
const setUser = useAuthStore((state) => state.setUser);
```

See the [official TypeScript usage guide](https://github.com/pmndrs/zustand?tab=readme-ov-file#typescript-usage) for advanced patterns.

- Do **not** use React Context for shared application state — use Zustand instead.
- React Context (`src/context/`) is reserved for static config or library providers (e.g. theme, i18n).

### Recipes

**Selecting a single slice** — component re-renders only when that value changes:
```ts
const user = useAuthStore((state) => state.user);
```

**Selecting multiple slices** — use `useShallow` to avoid re-renders when the object reference changes but values haven't:
```ts
import { useShallow } from 'zustand/react/shallow';

const { nuts, honey } = useBearStore(
  useShallow((state) => ({ nuts: state.nuts, honey: state.honey })),
);
```

**Do not fetch the entire store** — this causes the component to re-render on every state change:
```ts
// ❌ avoid
const state = useBearStore();
```

**Async actions** — call `set` when ready, Zustand doesn't care if actions are async:
```ts
interface FishState {
  fishies: Record<string, unknown>;
  fetch: (pond: string) => Promise<void>;
}

export const useFishStore = create<FishState>()((set) => ({
  fishies: {},
  fetch: async (pond) => {
    const response = await fetch(pond);
    set({ fishies: await response.json() });
  },
}));
```

**Reading state inside actions** — use `get()` to read current state without a subscription:
```ts
create<SoundState>()((set, get) => ({
  sound: 'grunt',
  action: () => {
    const sound = get().sound;
    // ...
  },
}));
```

**Overwriting state** — pass `true` as the second argument to `set` to replace rather than merge:
```ts
deleteEverything: () => set({}, true), // clears the entire store
```

**Accessing state outside components** — use `.getState()` and `.setState()` on the store hook directly:
```ts
const user = useAuthStore.getState().user;
useAuthStore.setState({ user: null });
```

---

## 4. Styling Conventions

- Use **CSS Modules**: one `ComponentName.module.css` per component, colocated in the component folder.
- Import and apply styles as:

```tsx
import styles from './Button.module.css';

<button className={styles.primaryButton}>Click</button>
```

- Class names inside `.module.css` files use **camelCase** (e.g. `.primaryButton`, `.iconWrapper`).
- **No inline styles** for layout or theming — use CSS Modules instead.
- Inline styles are acceptable only for truly dynamic runtime values (e.g. computed widths).
- Global styles (CSS reset, base typography, CSS custom properties) belong in `src/styles/`.

---

## 5. Routing

Use **`react-router-dom`** (v7) for all client-side routing. All route definitions live in a single file: `src/routes/Routes.tsx`. Do not scatter `<Route>` declarations across the component tree.

### 5.1 `src/routes/Routes.tsx`

Define every application route here using the `<Routes>` + `<Route>` API. This file is the single source of truth for the URL structure of the app.

```tsx
// src/routes/Routes.tsx
import { Routes, Route } from 'react-router-dom';
import { HomePage } from '../pages/HomePage/HomePage';
import { DashboardPage } from '../pages/DashboardPage/DashboardPage';
import { NotFoundPage } from '../pages/NotFoundPage/NotFoundPage';

export function AppRoutes() {
  return (
    <Routes>
      <Route path="/" element={<HomePage />} />
      <Route path="/dashboard" element={<DashboardPage />} />
      {/* Add new routes here */}
      <Route path="*" element={<NotFoundPage />} />
    </Routes>
  );
}
```

Rules:
- Always include a `path="*"` catch-all that renders a `NotFoundPage`.
- Nest child routes using `<Route>` children — do **not** spread them across multiple files.
- Use lazy loading (`React.lazy` + `<Suspense>`) for page-level routes when the bundle grows large, but keep the route declarations in this file.

### 5.2 `src/index.tsx` (entrypoint)

The entrypoint wraps `<App />` in `<BrowserRouter>`. This is the **only** place `BrowserRouter` is instantiated — never nest it or duplicate it.

```tsx
// src/index.tsx
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { BrowserRouter } from 'react-router-dom';
import { AppRoutes } from './routes/Routes';
import './styles/global.css';

const root = document.getElementById('root');
if (!root) throw new Error('Root element not found');

createRoot(root).render(
  <StrictMode>
    <BrowserRouter>
      <AppRoutes />
    </BrowserRouter>
  </StrictMode>,
);
```

Rules:
- `<BrowserRouter>` lives in `index.tsx` only — never inside `App.tsx` or any other component.
- `<StrictMode>` wraps the entire tree.
- Assert the root element exists before calling `createRoot` — do not use `!` non-null assertion.
- Import global CSS here, not inside `App.tsx`.

### 5.4 Navigation

Use `<Link>` and `<NavLink>` from `react-router-dom` for all internal navigation — never use `<a href>` for in-app links. Use the `useNavigate` hook for programmatic navigation.

```tsx
import { Link, NavLink, useNavigate } from 'react-router-dom';

// Declarative
<Link to="/dashboard">Go to Dashboard</Link>
<NavLink to="/dashboard" className={({ isActive }) => isActive ? styles.active : ''}>
  Dashboard
</NavLink>

// Programmatic
const navigate = useNavigate();
navigate('/dashboard');
```

---

## 6. API Calls

The API layer lives in `src/api/`. It is generated from the backend OpenAPI schema and must not be written by hand.

### 6.1 Structure

```
src/api/
  schema.d.ts   # auto-generated — do not edit; regenerate with `just frontend-gen-api-client`
  client.ts     # HTTP method helpers (get, post, put, delete, …); reads API_BASE_URL from REACT_APP_API_URL env var
```

### 6.2 `client.ts`

`client.ts` is the single place where HTTP method helpers are defined. Currently it exposes `get`; add `post`, `put`, `delete`, etc. here as the API grows — never call `fetch` directly outside this file.

```ts
const API_BASE_URL = process.env.REACT_APP_API_URL ?? 'http://localhost:3030/api';

export async function get(path: string): Promise<Response> {
  return fetch(API_BASE_URL + path);
}

// Add further helpers here as needed, e.g.:
// export async function post(path: string, body: unknown): Promise<Response> { ... }
// export async function put(path: string, body: unknown): Promise<Response> { ... }
// export async function del(path: string): Promise<Response> { ... }
```

Override the base URL at runtime by setting `REACT_APP_API_URL` in the environment. Falls back to `http://localhost:3030/api` for local development.

### 6.3 Making a typed API call

Always derive request and response types directly from `schema.d.ts` — never write them by hand.

```ts
import type { paths } from '../api/schema';
import { get } from '../api/client';

type EchoParams = paths['/echo']['get']['parameters']['query'];
type EchoResponse = paths['/echo']['get']['responses'][200]['content']['text/plain; charset=utf-8'];

const params: EchoParams = { message: 'Hello' };
const query = new URLSearchParams(params).toString();
const response = await get(`/echo?${query}`);
const text: EchoResponse = await response.text();
```

### 6.4 Rules

- **Never hardcode path strings** — always use paths that exist in `keyof paths` to stay in sync with the backend.
- **Always type inputs and outputs from `schema.d.ts`** — if a type would require writing it by hand, the schema needs to be regenerated instead.
- **Regenerate the schema** after any backend endpoint change: `just frontend-gen-api-client`.
- **Do not add `openapi-fetch` or similar wrappers** — the current thin `get()` helper is intentional; extend it only when there is a concrete need (POST, auth headers, etc.).