<script setup lang="ts">
import { ref, watch, nextTick } from 'vue'
import MarkdownIt from 'markdown-it'
import hljs from 'highlight.js'
import mermaid from 'mermaid'

import 'highlight.js/styles/github-dark.css'
import 'katex/dist/katex.min.css'

interface Props {
  content: string
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'todo-click', task: string): void
}>()

const containerRef = ref<HTMLElement | null>(null)
const renderedHtml = ref('')
let mermaidCounter = 0

// Initialize mermaid once
mermaid.initialize({
  startOnLoad: false,
  theme: 'neutral',
  securityLevel: 'loose',
})

// Initialize markdown-it with plugins
const md = new MarkdownIt({
  html: true,
  linkify: true,
  typographer: true,
  breaks: false,
})

// Plugins are loaded dynamically to handle CJS modules
async function loadPlugins() {
  const highlightjs = (await import('markdown-it-highlightjs')).default
  const katexPlugin = (await import('markdown-it-katex')).default
  md.use(highlightjs, { hljs, auto: true, code: true })
  md.use(katexPlugin)
}

const pluginsReady = loadPlugins()

/**
 * Pre-process content before markdown-it parsing:
 * - Extract mermaid code blocks (avoid highlight.js processing)
 * - Convert !video[url] to <video> elements
 * - Wrap #todo patterns in styled spans
 */
function preprocess(content: string): {
  processed: string
  mermaidBlocks: Map<string, string>
} {
  const mermaidBlocks = new Map<string, string>()
  let processed = content

  // Extract mermaid fenced blocks
  processed = processed.replace(
    /```mermaid\n([\s\S]*?)```/g,
    (_, code: string) => {
      const id = `__MERMAID_PLACEHOLDER_${mermaidBlocks.size}__`
      mermaidBlocks.set(id, code.trim())
      return id
    },
  )

  // Convert !video[url] to <video>
  processed = processed.replace(
    /!video\[([^\]]+)\]/g,
    (_, url: string) => {
      const safeUrl = url.replace(/"/g, '&quot;')
      return `<div class="post-video"><video src="${safeUrl}" controls playsinline preload="metadata" /></div>`
    },
  )

  // Wrap #todo patterns in styled HTML
  processed = processed.replace(
    /#todo\b([^#]*?)(?=#todo|$)/g,
    (_match, taskText: string) => {
      const task = taskText.trim() || '待办事项'
      const escapedTask = task.replace(/"/g, '&quot;')
      return (
        `<span class="todo-highlight" data-task="${escapedTask}">` +
        `<span class="todo-highlight__tag">#todo</span>` +
        `<span class="todo-highlight__text">${task}</span>` +
        `<button class="todo-highlight__subscribe" data-todo-task="${escapedTask}">订阅更新</button>` +
        `</span>`
      )
    },
  )

  return { processed, mermaidBlocks }
}

/**
 * Post-process rendered HTML:
 * - Replace mermaid placeholders with diagram containers
 */
function postprocess(
  html: string,
  mermaidBlocks: Map<string, string>,
): string {
  let result = html

  mermaidBlocks.forEach((code, id) => {
    // Placeholder may be wrapped in <p> tags by markdown-it
    const pRegex = new RegExp(`<p>${id}</p>`, 'g')
    result = result.replace(
      pRegex,
      `<div class="mermaid-diagram" data-mermaid="${encodeURIComponent(code)}"></div>`,
    )
    // Also handle bare placeholder (not in <p>)
    const bareRegex = new RegExp(id, 'g')
    result = result.replace(
      bareRegex,
      `<div class="mermaid-diagram" data-mermaid="${encodeURIComponent(code)}"></div>`,
    )
  })

  return result
}

/**
 * Render mermaid diagrams asynchronously after DOM update
 */
async function renderMermaidDiagrams() {
  if (!containerRef.value) return

  const diagrams = containerRef.value.querySelectorAll('.mermaid-diagram')
  if (diagrams.length === 0) return

  for (const el of Array.from(diagrams)) {
    const code = decodeURIComponent(el.getAttribute('data-mermaid') || '')
    if (!code.trim()) continue

    const id = `mermaid-${++mermaidCounter}-${Math.random().toString(36).slice(2, 8)}`

    try {
      const { svg } = await mermaid.render(id, code)
      el.innerHTML = svg
      el.classList.remove('mermaid-diagram')
      el.classList.add('mermaid-rendered')
    } catch (err) {
      console.warn('Mermaid render error:', err)
      el.innerHTML = `<pre class="mermaid-error"><code>${escapeHtml(code)}</code></pre>`
      el.classList.remove('mermaid-diagram')
      el.classList.add('mermaid-error')
    }
  }
}

/** Escape HTML entities for error fallback display */
function escapeHtml(str: string): string {
  return str
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
}

/**
 * Bind click events on #todo subscribe buttons via emit
 */
function bindTodoEvents() {
  if (!containerRef.value) return

  const buttons = containerRef.value.querySelectorAll(
    '.todo-highlight__subscribe',
  )
  buttons.forEach((btn) => {
    // Clone to remove stale listeners
    const newBtn = btn.cloneNode(true)
    btn.parentNode?.replaceChild(newBtn, btn)

    newBtn.addEventListener('click', (e) => {
      e.preventDefault()
      const task = (newBtn as HTMLElement).getAttribute('data-todo-task') || ''
      emit('todo-click', task)
    })
  })
}

/**
 * Main render pipeline: preprocess → markdown-it → postprocess → mermaid + events
 */
async function renderContent() {
  await pluginsReady

  if (!props.content) {
    renderedHtml.value = ''
    return
  }

  const { processed, mermaidBlocks } = preprocess(props.content)
  const html = md.render(processed)
  renderedHtml.value = postprocess(html, mermaidBlocks)

  nextTick(async () => {
    await renderMermaidDiagrams()
    bindTodoEvents()
  })
}

watch(() => props.content, renderContent, { immediate: true })
</script>

<template>
  <div ref="containerRef" class="markdown-content" v-html="renderedHtml" />
</template>

<style scoped>
/* --- Base --- */
.markdown-content {
  font-family: var(--font-body);
  font-size: var(--text-base);
  line-height: 1.8;
  color: var(--color-text-primary);
}

/* --- Responsive images --- */
.markdown-content :deep(img) {
  max-width: 100%;
  height: auto;
  border-radius: var(--radius-md);
}

/* --- Tables --- */
.markdown-content :deep(table) {
  width: 100%;
  border-collapse: collapse;
  margin-bottom: var(--space-4);
  font-size: var(--text-sm);
}

.markdown-content :deep(th),
.markdown-content :deep(td) {
  padding: var(--space-2) var(--space-3);
  border: 1px solid var(--color-border);
  text-align: left;
}

.markdown-content :deep(th) {
  background: var(--color-bg-sunken);
  font-weight: 600;
  color: var(--color-text-primary);
}

.markdown-content :deep(td) {
  color: var(--color-text-secondary);
}

/* --- Horizontal rule --- */
.markdown-content :deep(hr) {
  border: none;
  border-top: 1px solid var(--color-border);
  margin: var(--space-8) 0;
}

/* --- Paragraphs --- */
.markdown-content :deep(p) {
  margin-bottom: var(--space-4);
  color: var(--color-text-primary);
  font-size: var(--text-base);
  line-height: 1.8;
}

/* --- Headings --- */
.markdown-content :deep(h1) {
  font-family: var(--font-display);
  font-size: var(--text-3xl);
  font-weight: 700;
  margin-top: var(--space-8);
  margin-bottom: var(--space-4);
  color: var(--color-text-primary);
}

.markdown-content :deep(h2) {
  font-family: var(--font-display);
  font-size: var(--text-2xl);
  font-weight: 600;
  margin-top: var(--space-8);
  margin-bottom: var(--space-4);
  color: var(--color-text-primary);
}

.markdown-content :deep(h3) {
  font-family: var(--font-display);
  font-size: var(--text-xl);
  font-weight: 600;
  margin-top: var(--space-6);
  margin-bottom: var(--space-3);
  color: var(--color-text-primary);
}

.markdown-content :deep(h4) {
  font-family: var(--font-display);
  font-size: var(--text-lg);
  font-weight: 600;
  margin-top: var(--space-6);
  margin-bottom: var(--space-3);
  color: var(--color-text-primary);
}

/* --- Code blocks (highlight.js handles colors via .hljs) --- */
.markdown-content :deep(pre) {
  background: var(--color-code-bg, #24292e);
  padding: var(--space-4);
  border-radius: var(--radius-md);
  overflow-x: auto;
  margin-bottom: var(--space-4);
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  line-height: 1.6;
}

.markdown-content :deep(pre code) {
  background: none;
  padding: 0;
  color: inherit;
}

/* --- Inline code (not inside pre) --- */
.markdown-content :deep(:not(pre) > code) {
  font-family: var(--font-mono);
  font-size: 0.9em;
  background: var(--color-bg-sunken);
  padding: 2px var(--space-1);
  border-radius: var(--radius-sm);
  color: var(--color-accent);
}

/* --- Blockquote --- */
.markdown-content :deep(blockquote) {
  border-left: 3px solid var(--color-accent);
  padding-left: var(--space-4);
  margin: var(--space-4) 0;
  color: var(--color-text-secondary);
  font-style: italic;
}

/* --- Links --- */
.markdown-content :deep(a) {
  color: var(--color-accent);
  text-decoration: underline;
  text-underline-offset: 2px;
  transition: color var(--transition-fast);
}

.markdown-content :deep(a:hover) {
  color: var(--color-accent-hover);
}

/* --- Lists --- */
.markdown-content :deep(ul),
.markdown-content :deep(ol) {
  margin-bottom: var(--space-4);
  padding-left: var(--space-6);
}

.markdown-content :deep(li) {
  margin-bottom: var(--space-2);
}

/* --- Emphasis --- */
.markdown-content :deep(strong) {
  font-weight: 600;
  color: var(--color-text-primary);
}

.markdown-content :deep(em) {
  font-style: italic;
}

/* --- Mermaid diagrams --- */
.markdown-content :deep(.mermaid-diagram) {
  margin: var(--space-6) 0;
  text-align: center;
  padding: var(--space-4);
  background: var(--color-bg-elevated);
  border: 1px solid var(--color-border-light);
  border-radius: var(--radius-md);
}

.markdown-content :deep(.mermaid-rendered) {
  margin: var(--space-6) 0;
  text-align: center;
}

.markdown-content :deep(.mermaid-rendered svg) {
  max-width: 100%;
  height: auto;
}

.markdown-content :deep(.mermaid-error) {
  margin: var(--space-4) 0;
  padding: var(--space-4);
  background: rgba(196, 62, 62, 0.08);
  border: 1px solid rgba(196, 62, 62, 0.2);
  border-radius: var(--radius-md);
  color: var(--color-error);
  font-size: var(--text-sm);
  font-family: var(--font-mono);
}

/* --- Video embeds --- */
.markdown-content :deep(.post-video) {
  margin: var(--space-6) 0;
  border-radius: var(--radius-lg);
  overflow: hidden;
  background: #000;
  border: 1px solid var(--color-border-light);
  transition: box-shadow var(--transition-base);
}

.markdown-content :deep(.post-video:hover) {
  box-shadow: var(--shadow-md);
}

.markdown-content :deep(.post-video video) {
  display: block;
  width: 100%;
  height: auto;
  max-height: 520px;
  object-fit: contain;
}

/* --- #todo highlight --- */
.markdown-content :deep(.todo-highlight) {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  background: linear-gradient(
    135deg,
    rgba(196, 93, 62, 0.08),
    rgba(196, 93, 62, 0.15)
  );
  border: 1px solid rgba(196, 93, 62, 0.2);
  border-radius: var(--radius-md);
  padding: var(--space-1) var(--space-3);
  margin: 0 var(--space-1);
  font-size: var(--text-sm);
  vertical-align: baseline;
  transition: all var(--transition-fast);
}

.markdown-content :deep(.todo-highlight:hover) {
  background: linear-gradient(
    135deg,
    rgba(196, 93, 62, 0.12),
    rgba(196, 93, 62, 0.2)
  );
  border-color: rgba(196, 93, 62, 0.3);
  box-shadow: var(--shadow-sm);
}

.markdown-content :deep(.todo-highlight__tag) {
  font-weight: 600;
  color: var(--color-accent);
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.markdown-content :deep(.todo-highlight__text) {
  color: var(--color-text-primary);
  font-weight: 500;
}

.markdown-content :deep(.todo-highlight__subscribe) {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 2px var(--space-2);
  background: var(--color-accent);
  color: white;
  border: none;
  border-radius: var(--radius-sm);
  font-size: var(--text-xs);
  font-weight: 500;
  font-family: var(--font-body);
  cursor: pointer;
  transition: all var(--transition-fast);
  opacity: 0;
  transform: scale(0.95);
}

.markdown-content :deep(.todo-highlight:hover .todo-highlight__subscribe) {
  opacity: 1;
  transform: scale(1);
}

.markdown-content :deep(.todo-highlight__subscribe:hover) {
  background: var(--color-accent-hover);
  transform: scale(1.05);
}

.markdown-content :deep(.todo-highlight__subscribe:active) {
  transform: scale(0.98);
}

/* --- Responsive --- */
@media (max-width: 767px) {
  .markdown-content :deep(.todo-highlight) {
    flex-wrap: wrap;
    gap: var(--space-1);
  }

  .markdown-content :deep(.todo-highlight__subscribe) {
    opacity: 1;
    transform: scale(1);
    width: 100%;
    justify-content: center;
    margin-top: var(--space-1);
  }

  .markdown-content :deep(.post-video video) {
    max-height: 280px;
  }
}
</style>

<!-- Light mode highlight.js overrides (not scoped — overrides global github-dark.css) -->
<style>
[data-theme="light"] .markdown-content pre {
  background: #f6f8fa !important;
  color: #24292e !important;
}

[data-theme="light"] .markdown-content pre code {
  color: #24292e !important;
}

[data-theme="light"] .markdown-content pre .hljs-comment,
[data-theme="light"] .markdown-content pre .hljs-quote {
  color: #6a737d !important;
  font-style: italic !important;
}

[data-theme="light"] .markdown-content pre .hljs-keyword,
[data-theme="light"] .markdown-content pre .hljs-selector-tag {
  color: #d73a49 !important;
}

[data-theme="light"] .markdown-content pre .hljs-string,
[data-theme="light"] .markdown-content pre .hljs-addition {
  color: #032f62 !important;
}

[data-theme="light"] .markdown-content pre .hljs-number,
[data-theme="light"] .markdown-content pre .hljs-literal,
[data-theme="light"] .markdown-content pre .hljs-variable,
[data-theme="light"] .markdown-content pre .hljs-template-variable {
  color: #005cc5 !important;
}

[data-theme="light"] .markdown-content pre .hljs-built_in,
[data-theme="light"] .markdown-content pre .hljs-type,
[data-theme="light"] .markdown-content pre .hljs-function,
[data-theme="light"] .markdown-content pre .hljs-title {
  color: #6f42c1 !important;
}

[data-theme="light"] .markdown-content pre .hljs-attr,
[data-theme="light"] .markdown-content pre .hljs-attribute,
[data-theme="light"] .markdown-content pre .hljs-meta {
  color: #005cc5 !important;
}

[data-theme="light"] .markdown-content pre .hljs-tag,
[data-theme="light"] .markdown-content pre .hljs-name {
  color: #22863a !important;
}

[data-theme="light"] .markdown-content pre .hljs-selector-class,
[data-theme="light"] .markdown-content pre .hljs-selector-id {
  color: #6f42c1 !important;
}

[data-theme="light"] .markdown-content pre .hljs-section {
  color: #005cc5 !important;
  font-weight: bold !important;
}

[data-theme="light"] .markdown-content pre .hljs-symbol {
  color: #e36209 !important;
}

[data-theme="light"] .markdown-content pre .hljs-bullet {
  color: #735c0f !important;
}

[data-theme="light"] .markdown-content pre .hljs-deletion {
  color: #b31d28 !important;
}

[data-theme="light"] .markdown-content pre .hljs-regexp,
[data-theme="light"] .markdown-content pre .hljs-link {
  color: #032f62 !important;
}

[data-theme="light"] .markdown-content pre .hljs-emphasis {
  font-style: italic !important;
}

[data-theme="light"] .markdown-content pre .hljs-strong {
  font-weight: bold !important;
}
</style>
