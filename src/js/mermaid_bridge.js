async function wait_element(id, attempts = 60) {
  for (let i = 0; i < attempts; i += 1) {
    const el = document.getElementById(id);
    if (el instanceof HTMLElement) return el;
    await new Promise((resolve) => requestAnimationFrame(resolve));
  }
  return null;
}

async function wait_mermaid(attempts = 400) {
  for (let i = 0; i < attempts; i += 1) {
    const mermaid = window.mermaid;
    if (mermaid) return mermaid;
    await new Promise((resolve) => requestAnimationFrame(resolve));
  }
  return null;
}

const render_version_by_id = new Map();
const rendered_base_ids = new Set();
const MERMAID_BASE_CONFIG = {
  securityLevel: "loose",
  startOnLoad: false,
};
let current_app_theme = null;
let current_mermaid_theme = null;
let is_theme_listener_bound = false;

function resolve_app_theme(theme) {
  return theme === "dark" ? "dark" : "light";
}

function map_mermaid_theme(app_theme) {
  return app_theme === "dark" ? "redux-dark" : "base";
}

function read_document_theme() {
  const root = document.documentElement;
  if (root && root.classList.contains("dark")) return "dark";
  return "light";
}

function apply_mermaid_theme(mermaid, app_theme) {
  const next_theme = map_mermaid_theme(app_theme);
  if (current_mermaid_theme === next_theme) return false;
  mermaid.initialize({
    ...MERMAID_BASE_CONFIG,
    theme: next_theme,
  });
  current_app_theme = app_theme;
  current_mermaid_theme = next_theme;
  return true;
}

async function rerender_all_diagrams() {
  const base_ids = Array.from(rendered_base_ids);
  await Promise.all(
    base_ids.map((base_id) => js_render_mermaid_by_base_id(base_id))
  );
}

function bind_theme_change_listener() {
  if (is_theme_listener_bound) return;
  document.addEventListener("app:theme-changed", (event) => {
    const app_theme = resolve_app_theme(event?.detail?.theme);
    void js_set_mermaid_theme(app_theme);
  });
  is_theme_listener_bound = true;
}

function begin_render(base_id) {
  const next = (render_version_by_id.get(base_id) ?? 0) + 1;
  render_version_by_id.set(base_id, next);
  return next;
}

function is_current_render(base_id, version) {
  return render_version_by_id.get(base_id) === version;
}

function set_error(output, error) {
  output.innerHTML = `<pre style="color:#c00;white-space:pre-wrap;">Mermaid render error:\n${String(
    error
  )}</pre>`;
}

export async function js_set_mermaid_theme(theme) {
  const mermaid = await wait_mermaid();
  if (!mermaid) return null;
  const app_theme = resolve_app_theme(theme);
  const theme_changed = apply_mermaid_theme(mermaid, app_theme);
  if (!theme_changed) return current_mermaid_theme;
  await rerender_all_diagrams();
  return current_mermaid_theme;
}

export async function js_render_mermaid_by_base_id(base_id) {
  if (!base_id) return null;
  bind_theme_change_listener();
  rendered_base_ids.add(base_id);
  const version = begin_render(base_id);

  const mermaid = await wait_mermaid();
  if (!mermaid) return null;
  const app_theme = current_app_theme ?? read_document_theme();
  apply_mermaid_theme(mermaid, app_theme);
  if (!is_current_render(base_id, version)) return null;

  const source = await wait_element(`${base_id}-src`);
  if (!source) return null;
  if (!is_current_render(base_id, version)) return null;

  const output = await wait_element(`${base_id}-out`);
  if (!output) return null;
  if (!is_current_render(base_id, version)) return null;

  const definition = (source.textContent || "").trim();
  if (!definition) return null;

  try {
    const render_id = `${base_id}-svg-v${version}`;
    const { svg } = await mermaid.render(render_id, definition);
    if (!is_current_render(base_id, version)) return null;
    output.innerHTML = svg;
    return svg;
  } catch (error) {
    if (!is_current_render(base_id, version)) return null;
    set_error(output, error);
    return null;
  }
}
