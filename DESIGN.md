# Design System: Developer Minimalism (Vercel/Linear Aesthetic)

## 1. Visual Theme & Atmosphere

This design system is inspired by top-tier design engineers and modern full-stack leaders (Paco Coursey, Pedro Duarte, Lee Robinson). The atmosphere is "invisible design" — the interface gets completely out of the way to let the content shine. It feels like a perfectly configured code editor or a high-end digital manuscript. 

Unlike traditional blogs that use cards, drop shadows, and large hero images, this style relies entirely on **typography, spacing, and extreme restraint**. It is the "Vercel/Linear aesthetic" applied to personal spaces: crisp, fast, monochromatic, and highly intentional.

**Key Characteristics:**
- **Content IS the UI**: No decorative boxes, no unnecessary borders, no background patterns.
- **Monochromatic Base**: Pure black and white, or extremely dark/light grays (Zinc/Neutral scale in Tailwind).
- **High Contrast vs. Low Contrast**: Pure black/white for primary text, but aggressively muted grays for secondary text (dates, views, metadata).
- **Micro-interactions**: Hover states are subtle and fast (e.g., text color brightening, or a very faint background highlight).
- **Typography-Driven**: System fonts (Inter, Geist, San Francisco) mixed with Monospace (Geist Mono, JetBrains Mono) for technical accents.

## 2. Color Palette & Roles

The palette is strictly grayscale. Accent colors are rarely used, and if they are, they are tiny indicators (like a 6px status dot).

### Light Mode
- **Background**: `bg-white` (`#ffffff`) or `bg-zinc-50` (`#fafafa`)
- **Primary Text**: `text-zinc-900` (`#18181b`) or `text-black`
- **Secondary Text**: `text-zinc-500` (`#71717a`) - Used for dates, descriptions, metadata
- **Subtle Borders**: `border-zinc-200` (`#e4e4e7`)
- **Hover Backgrounds**: `bg-zinc-100` (`#f4f4f5`)

### Dark Mode (Crucial for this style)
- **Background**: `bg-black` (`#000000`) or `bg-zinc-950` (`#09090b`)
- **Primary Text**: `text-zinc-100` (`#f4f4f5`) or `text-white`
- **Secondary Text**: `text-zinc-400` (`#a1a1aa`)
- **Subtle Borders**: `border-zinc-800` (`#27272a`) or `border-white/10`
- **Hover Backgrounds**: `bg-zinc-900` (`#18181b`) or `bg-white/5`

## 3. Typography Rules

### Font Family
- **Sans-serif (Primary)**: `Inter`, `Geist`, or system-ui. Clean, geometric, highly legible.
- **Monospace (Accent/Code)**: `Geist Mono`, `JetBrains Mono`. Used for dates, tags, code blocks, and sometimes navigation.
- **Serif (Optional Accent)**: `Newsreader` or `Playfair Display`. Used sparingly for article titles or logos to add a touch of editorial elegance (optional, but adds personality).

### Hierarchy
This style avoids massive, shouting headlines. Everything is scaled down and refined.

| Role | Font | Size | Weight | Line Height | Tracking (Spacing) |
|------|------|------|--------|-------------|--------------------|
| Site Title/Logo | Sans or Serif | 18px-20px | 500/600 | 1.2 | tight (-0.02em) |
| Page Title (H1) | Sans | 24px-32px | 600 (Semibold) | 1.2 | tight (-0.02em) |
| Section Title (H2)| Sans | 18px-20px | 500 (Medium) | 1.4 | normal |
| Body Text | Sans | 16px | 400 (Regular) | 1.6 - 1.75 | normal |
| Metadata/Dates | Mono | 13px-14px | 400 (Regular) | 1.0 | wide (0.05em) |

### Principles
- **Muted Metadata**: Dates, view counts, and tags should be significantly lighter than the title (e.g., `text-zinc-500`).
- **Tight Headings, Loose Body**: Headings have tight line-height (`leading-tight`) and tracking (`tracking-tight`). Body text has generous line-height (`leading-relaxed` or `leading-7`) for readability.

## 4. Component Stylings

### Links & Navigation
- **Default**: Inherit text color or use secondary color. No underlines by default.
- **Hover**: Change color to primary (e.g., `zinc-500` to `zinc-900`) or add a subtle underline (`underline underline-offset-4 decoration-zinc-300`).
- **Active State**: Bold or primary color to indicate current page.

### Blog Post Lists (The "Paco/LeeRob" Pattern)
- **No Cards**: Do not put blog posts in boxes or cards.
- **Inline Layout**: Flex container. Title on the left (primary text), Date/Views on the right (secondary text, mono font).
- **Hover Effect**: The entire row might get a subtle background (`hover:bg-zinc-100 dark:hover:bg-zinc-900`) with negative margins to keep text aligned, OR simply dim the non-hovered items (group-hover effects).

### Buttons
- **Primary**: Black background, white text (`bg-black text-white dark:bg-white dark:text-black`). Small padding (`px-4 py-2`), rounded corners (`rounded-md` or `rounded-full`).
- **Secondary/Ghost**: Transparent background, primary text. Hover gives a subtle gray background.

## 5. Layout Principles

### The "Reading Column"
- **Max Width**: The entire site content is constrained to a narrow column. `max-w-[65ch]` (approx 640px-700px) is the golden rule.
- **Centering**: `mx-auto` to center the column on the screen.
- **Padding**: Generous horizontal padding on mobile (`px-6`), and vertical padding (`py-12 md:py-24`).

### Whitespace (Negative Space)
- Spacing is used instead of lines to separate sections.
- Use large margins between distinct sections (e.g., `mb-24` or `mb-32`).
- Use tight margins between related items (e.g., `gap-4` in a list of posts).

## 6. Depth & Elevation

**FLAT. Absolutely Flat.**
- **No Drop Shadows**: Do not use `shadow-md`, `shadow-lg`, etc.
- **Elevation via Borders**: If something must be separated (like a command palette or a sticky header), use a 1px subtle border (`border border-zinc-200 dark:border-zinc-800`) and a blur effect (`backdrop-blur-md bg-white/80`).
- **Elevation via Brightness**: In dark mode, elevated surfaces are slightly lighter gray (`bg-zinc-900`) than the background (`bg-black`).

## 7. Do's and Don'ts

### Do
- Use `text-zinc-500` (or similar) heavily for anything that isn't the main content.
- Align everything perfectly to the left edge of the reading column.
- Use `flex` with `justify-between` and `items-baseline` for list items (Title --- Date).
- Implement a flawless Dark Mode.
- Use `underline-offset-4` or `underline-offset-8` when using underlines.

### Don't
- Don't use cards or boxes for blog posts.
- Don't use primary colors (blue, red, green) for text or buttons unless it's a critical error/success state.
- Don't use large hero images or illustrations.
- Don't use bold weights (700+) excessively. Stick to 400, 500, and 600.
- Don't use drop shadows (`shadow-md`, etc.).

## 8. Responsive Behavior
- **Mobile First**: The layout is naturally mobile-friendly because it's a single column.
- **List Items on Mobile**: A row with "Title [Space] Date" might get too squished on mobile. Change it to `flex-col` on mobile, and `flex-row` on `md:` screens.
- **Navigation**: Keep it simple. A few links at the top. No complex hamburger menus needed for 3-4 links.

## 9. Agent Prompt Guide

### Tailwind Class Reference
- **Container**: `max-w-[65ch] mx-auto px-6 py-12 md:py-24`
- **Primary Text**: `text-zinc-900 dark:text-zinc-100`
- **Secondary Text**: `text-zinc-500 dark:text-zinc-400`
- **Mono Accent**: `font-mono text-sm tracking-tight`
- **Post List Row**: `flex flex-col md:flex-row md:items-baseline justify-between gap-2 md:gap-4 py-3`
- **Subtle Hover**: `transition-colors hover:text-zinc-900 dark:hover:text-zinc-100`

### Example Component Prompts
- "Create a blog post list item. No borders, no cards. Title on the left in zinc-900 font-medium. Date on the right in zinc-500 font-mono text-sm. On hover, the title should get an underline with underline-offset-4."
- "Design a site header. Max width 65ch. Logo on the left (font-medium), and a nav on the right with 3 links. Links should be zinc-500 and turn zinc-900 on hover. No background, no shadow."
- "Create a hero section. H1 should be 24px (text-2xl), font-semibold, tracking-tight. Below it, a paragraph in zinc-500, leading-relaxed. No images, just perfect typography."
