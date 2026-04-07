export function js_apply_theme(theme) {
  if (!theme) return;
  const root = document.documentElement;
  root.setAttribute("class", theme);
}

export function js_apply_lang(lang) {
  if (!lang) return;
  const root = document.documentElement;
  root.setAttribute("lang", lang);
}
