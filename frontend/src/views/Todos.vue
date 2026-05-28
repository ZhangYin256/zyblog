<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import {
  NSpin,
  NEmpty,
  NButton,
  NBadge,
  useMessage,
} from 'naive-ui'
import { useTodos, type TodoItem } from '../composables/useTodos'
import api from '../lib/api'

const message = useMessage()
const { todos, loading, fetchTodos, completeTodo } = useTodos()

// Post title lookup: post_id → title
const postTitles = ref<Record<number, string>>({})
const completingId = ref<number | null>(null)

// Filter state
type FilterMode = 'all' | 'pending' | 'done'
const filterMode = ref<FilterMode>('all')

// Fetch post titles for display
async function fetchPostTitles() {
  try {
    const { data } = await api.get('/api/v1/posts', { params: { per_page: 1000 } })
    const map: Record<number, string> = {}
    for (const post of data.items ?? []) {
      map[post.id] = post.title
    }
    postTitles.value = map
  } catch {
    // Silently fail — post_id will be shown instead
  }
}

// Filtered todos
const filteredTodos = computed(() => {
  switch (filterMode.value) {
    case 'pending':
      return todos.value.filter((t) => !t.completed)
    case 'done':
      return todos.value.filter((t) => t.completed)
    default:
      return todos.value
  }
})

// Group todos by post_id
interface PostGroup {
  postId: number
  postTitle: string
  items: TodoItem[]
}

const groupedTodos = computed<PostGroup[]>(() => {
  const map = new Map<number, TodoItem[]>()
  for (const todo of filteredTodos.value) {
    const arr = map.get(todo.post_id) ?? []
    arr.push(todo)
    map.set(todo.post_id, arr)
  }
  return Array.from(map.entries()).map(([postId, items]) => ({
    postId,
    postTitle: postTitles.value[postId] ?? `文章 #${postId}`,
    items,
  }))
})

// Stats
const totalCount = computed(() => todos.value.length)
const pendingCount = computed(() => todos.value.filter((t) => !t.completed).length)
const doneCount = computed(() => todos.value.filter((t) => t.completed).length)

// Mark a TODO as completed
async function handleComplete(todoId: number) {
  completingId.value = todoId
  const result = await completeTodo(todoId)
  completingId.value = null
  if (result.success) {
    message.success('已标记为完成')
  } else {
    message.error(result.error || '操作失败')
  }
}

onMounted(() => {
  fetchTodos()
  fetchPostTitles()
})
</script>

<template>
  <div class="todos-admin">
    <!-- Header -->
    <header class="todos-admin__header">
      <div class="todos-admin__header-info">
        <h1 class="todos-admin__title">TODO 管理</h1>
        <p class="todos-admin__subtitle">管理文章中的待办事项</p>
      </div>
    </header>

    <!-- Filter Tabs -->
    <div class="todos-admin__filters">
      <button
        class="filter-tab"
        :class="{ 'filter-tab--active': filterMode === 'all' }"
        @click="filterMode = 'all'"
      >
        全部
        <span class="filter-tab__count">{{ totalCount }}</span>
      </button>
      <button
        class="filter-tab"
        :class="{ 'filter-tab--active': filterMode === 'pending' }"
        @click="filterMode = 'pending'"
      >
        未完成
        <span class="filter-tab__count">{{ pendingCount }}</span>
      </button>
      <button
        class="filter-tab"
        :class="{ 'filter-tab--active': filterMode === 'done' }"
        @click="filterMode = 'done'"
      >
        已完成
        <span class="filter-tab__count">{{ doneCount }}</span>
      </button>
    </div>

    <!-- Loading -->
    <div v-if="loading && todos.length === 0" class="todos-admin__loading">
      <n-spin size="large" />
    </div>

    <!-- Empty -->
    <div v-else-if="filteredTodos.length === 0" class="todos-admin__empty">
      <n-empty :description="filterMode === 'all' ? '暂无 TODO 项' : '该分类下暂无 TODO 项'" />
    </div>

    <!-- Grouped TODO List -->
    <div v-else class="todo-groups">
      <section
        v-for="group in groupedTodos"
        :key="group.postId"
        class="todo-group"
      >
        <h2 class="todo-group__title">
          <span class="todo-group__post-title">{{ group.postTitle }}</span>
          <span class="todo-group__count">{{ group.items.length }} 项</span>
        </h2>

        <div class="todo-group__list">
          <div
            v-for="todo in group.items"
            :key="todo.id"
            class="todo-card"
            :class="{ 'todo-card--done': todo.completed }"
          >
            <div class="todo-card__body">
              <div class="todo-card__status">
                <span v-if="todo.completed" class="todo-card__check todo-card__check--done">✓</span>
                <span v-else class="todo-card__check">○</span>
              </div>
              <div class="todo-card__info">
                <span class="todo-card__title" :class="{ 'todo-card__title--done': todo.completed }">
                  {{ todo.title }}
                </span>
              </div>
              <n-badge
                :value="todo.subscriber_count"
                :max="99"
                class="todo-card__badge"
              >
                <span class="todo-card__badge-label">订阅</span>
              </n-badge>
            </div>
            <div class="todo-card__actions">
              <n-button
                v-if="!todo.completed"
                type="primary"
                size="small"
                :loading="completingId === todo.id"
                @click.stop="handleComplete(todo.id)"
              >
                完成
              </n-button>
              <span v-else class="todo-card__done-label">已完成</span>
            </div>
          </div>
        </div>
      </section>
    </div>
  </div>
</template>

<style scoped>
.todos-admin {
  animation: fadeIn var(--transition-slow) ease-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* --- Header --- */
.todos-admin__header {
  margin-bottom: var(--space-8);
  padding-bottom: var(--space-6);
  border-bottom: 1px solid var(--color-border);
}

.todos-admin__header-info {
  flex: 1;
}

.todos-admin__title {
  font-family: var(--font-display);
  font-size: var(--text-4xl);
  font-weight: 700;
  color: var(--color-text-primary);
  letter-spacing: -0.03em;
  margin-bottom: var(--space-2);
}

.todos-admin__subtitle {
  font-size: var(--text-lg);
  color: var(--color-text-tertiary);
  font-weight: 300;
}

/* --- Filter Tabs --- */
.todos-admin__filters {
  display: flex;
  gap: var(--space-2);
  margin-bottom: var(--space-8);
}

.filter-tab {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-4);
  font-size: var(--text-sm);
  font-weight: 500;
  color: var(--color-text-secondary);
  background: transparent;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-fast);
  font-family: var(--font-body);
}

.filter-tab:hover {
  background-color: var(--color-bg-sunken);
  color: var(--color-text-primary);
}

.filter-tab--active {
  background-color: var(--color-accent);
  color: var(--color-text-inverse);
  border-color: var(--color-accent);
}

.filter-tab--active:hover {
  background-color: var(--color-accent-hover);
  color: var(--color-text-inverse);
}

.filter-tab__count {
  font-size: var(--text-xs);
  font-variant-numeric: tabular-nums;
  opacity: 0.8;
}

/* --- Loading / Empty --- */
.todos-admin__loading,
.todos-admin__empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 300px;
  gap: var(--space-4);
}

/* --- Grouped List --- */
.todo-groups {
  display: flex;
  flex-direction: column;
  gap: var(--space-10);
}

.todo-group__title {
  display: flex;
  align-items: baseline;
  gap: var(--space-3);
  margin-bottom: var(--space-4);
  padding-bottom: var(--space-2);
  border-bottom: 1px solid var(--color-border-light);
}

.todo-group__post-title {
  font-family: var(--font-display);
  font-size: var(--text-xl);
  font-weight: 600;
  color: var(--color-text-primary);
}

.todo-group__count {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  font-weight: 400;
}

.todo-group__list {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

/* --- Todo Card --- */
.todo-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-4);
  padding: var(--space-4) var(--space-4);
  border-radius: var(--radius-md);
  transition: background-color var(--transition-fast);
}

.todo-card:hover {
  background-color: var(--color-bg-sunken);
}

.todo-card--done {
  opacity: 0.65;
}

.todo-card__body {
  flex: 1;
  min-width: 0;
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.todo-card__status {
  flex-shrink: 0;
}

.todo-card__check {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 22px;
  height: 22px;
  border-radius: var(--radius-sm);
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
  border: 1.5px solid var(--color-border);
  background: var(--color-bg-elevated);
}

.todo-card__check--done {
  background: var(--color-success);
  border-color: var(--color-success);
  color: #fff;
}

.todo-card__info {
  flex: 1;
  min-width: 0;
}

.todo-card__title {
  font-size: var(--text-base);
  color: var(--color-text-primary);
  font-weight: 500;
}

.todo-card__title--done {
  text-decoration: line-through;
  color: var(--color-text-tertiary);
}

.todo-card__badge {
  flex-shrink: 0;
}

.todo-card__badge-label {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  padding: var(--space-1) var(--space-2);
  background: var(--color-bg-sunken);
  border-radius: var(--radius-sm);
}

.todo-card__actions {
  flex-shrink: 0;
  display: flex;
  align-items: center;
}

.todo-card__done-label {
  font-size: var(--text-xs);
  color: var(--color-success);
  font-weight: 500;
}

/* --- Responsive --- */
@media (max-width: 767px) {
  .todos-admin__title {
    font-size: var(--text-3xl);
  }

  .todos-admin__filters {
    flex-wrap: wrap;
  }

  .todo-card {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-3);
    padding: var(--space-4) 0;
  }

  .todo-card__actions {
    width: 100%;
  }

  .todo-card__actions .n-button {
    width: 100%;
  }
}
</style>
