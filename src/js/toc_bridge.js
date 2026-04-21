let cleanup_toc_scrollspy = null;

function resolve_active_id(scroll_container, heading_ids, top_offset) {
  if (!Array.isArray(heading_ids) || heading_ids.length === 0) return "";

  const container_top = scroll_container.getBoundingClientRect().top;
  let active_id = "";

  for (const id of heading_ids) {
    const heading = document.getElementById(id);
    if (!(heading instanceof HTMLElement)) continue;

    const top = heading.getBoundingClientRect().top - container_top;
    if (top <= top_offset) {
      active_id = id;
      continue;
    }

    if (!active_id) active_id = id;
    break;
  }

  return active_id || heading_ids[0] || "";
}

function apply_active_styles(active_id) {
  const links = document.querySelectorAll("[data-toc-id]");
  links.forEach((link) => {
    if (!(link instanceof HTMLElement)) return;
    const is_active = link.dataset.tocId === active_id;

    link.classList.toggle("text-foreground", is_active);
    link.classList.toggle("font-medium", is_active);
    link.classList.toggle("opacity-100", is_active);

    link.classList.toggle("text-muted-foreground", !is_active);
    link.classList.toggle("opacity-65", !is_active);
  });
}

export function js_bind_toc_scrollspy(scroll_container_id, heading_ids, top_offset = 24) {
  if (typeof cleanup_toc_scrollspy === "function") {
    cleanup_toc_scrollspy();
    cleanup_toc_scrollspy = null;
  }

  const scroll_container = document.getElementById(scroll_container_id);
  if (!(scroll_container instanceof HTMLElement)) return;

  const update = () => {
    const active_id = resolve_active_id(scroll_container, heading_ids, top_offset);
    apply_active_styles(active_id);
  };

  scroll_container.addEventListener("scroll", update, { passive: true });
  window.addEventListener("resize", update);
  requestAnimationFrame(update);

  cleanup_toc_scrollspy = () => {
    scroll_container.removeEventListener("scroll", update);
    window.removeEventListener("resize", update);
  };
}
