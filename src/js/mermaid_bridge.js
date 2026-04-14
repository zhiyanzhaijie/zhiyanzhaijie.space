async function wait_text_source(base_id, attempts = 400) {
  const source_id = `${base_id}-src`;
  for (let i = 0; i < attempts; i += 1) {
    const el = document.getElementById(source_id);
    if (el instanceof HTMLElement) return el;
    await new Promise((resolve) => requestAnimationFrame(resolve));
  }
  return null;
}

async function wait_output(base_id, attempts = 400) {
  const output_id = `${base_id}-out`;
  for (let i = 0; i < attempts; i += 1) {
    const el = document.getElementById(output_id);
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
const MERMAID_NEUTRAL_THEME = {
  theme: "base",
  securityLevel: "loose",
  startOnLoad: false,
  themeCSS: `
    .labelBkg,
    .edgeLabel .labelBkg,
    .edgeLabel .label,
    .edgeLabel .label rect {
      background-color: transparent !important;
      fill: transparent !important;
      stroke: transparent !important;
    }
  `,
  themeVariables: {
    background: "transparent",
    primaryColor: "rgba(85,85,85,0.08)",
    primaryBorderColor: "#555555",
    primaryTextColor: "#555555",
    secondaryColor: "rgba(85,85,85,0.06)",
    secondaryBorderColor: "#555555",
    secondaryTextColor: "#555555",
    tertiaryColor: "rgba(85,85,85,0.04)",
    tertiaryBorderColor: "#555555",
    tertiaryTextColor: "#555555",
    mainBkg: "rgba(85,85,85,0.08)",
    secondBkg: "rgba(85,85,85,0.06)",
    tertiaryBkg: "rgba(85,85,85,0.04)",
    lineColor: "#555555",
    textColor: "#555555",
    nodeTextColor: "#555555",
    labelTextColor: "#555555",
    edgeLabelBackground: "transparent",
    edgeLabelBgColor: "transparent",
    labelBackground: "transparent",
    labelBkgColor: "transparent",
    labelBkgBackground: "transparent",
    labelBoxBkgColor: "transparent",
    clusterBkg: "rgba(85,85,85,0.06)",
    clusterBorder: "#555555",
    actorBkg: "rgba(85,85,85,0.06)",
    actorBorder: "#555555",
    actorTextColor: "#555555",
    noteBkgColor: "rgba(85,85,85,0.06)",
    noteBorderColor: "#555555",
    noteTextColor: "#555555",
    sequenceNumberColor: "#555555",
    activationBkgColor: "rgba(85,85,85,0.06)",
    activationBorderColor: "#555555",
  },
};

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

export async function js_render_mermaid_by_base_id(base_id) {
  if (!base_id) return null;
  const version = begin_render(base_id);

  const mermaid = await wait_mermaid();
  if (!mermaid) return null;
  if (!is_current_render(base_id, version)) return null;

  const source = await wait_text_source(base_id);
  if (!source) return null;
  if (!is_current_render(base_id, version)) return null;

  const output = await wait_output(base_id);
  if (!output) return null;
  if (!is_current_render(base_id, version)) return null;

  const definition = (source.textContent || "").trim();
  output.innerHTML = "";
  if (!definition) return null;

  try {
    mermaid.initialize(MERMAID_NEUTRAL_THEME);
    const render_id = `${base_id}-svg`;
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
