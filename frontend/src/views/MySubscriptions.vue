<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import {
  NSpin,
  NEmpty,
  NTag,
  NBadge,
} from 'naive-ui'
import { useTodos, type TodoItem } from '../composables/useTodos'
import { useTags } from '../composables/useTags'
import api from '../lib/api'

const { todos, loading: todosLoading, fetchTodos } = useTodos()
const { tags, loading: tagsLoading, fetchTags } = useTags()

// Post title lookup
const postTitles = ref<Record<number, string>>({})

async function fetchPostTitles() {
  try {
    const { data } = await api.get('/api/v1/posts', { params: { per_page: 1000 } })
    const map: Record<number, string> = {}
    for (const post of data.items ?? []) {
      map[post.id] = post.title
    }
    postTitles.value = map
  } catch {
    // Silent fail — post_id will be shown instead
  }
}

const loading = computed(() => todosLoading.value || tagsLoading.value)

// Filter state
type TabMode = 'todos' | 'tags'
const activeTab = ref<TabMode>('todos')

// Group todos by post
interface PostGroup {
  postId: number
  postTitle: string
  items: TodoItem[]
}

const groupedTodos = computed<PostGroup[]>(() => {
  const map = new Map<number, TodoItem[]>()
  for (const todo of todos.value) {
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

onMounted(() => {
  fetchTodos()
  fetchTags()
  fetchPostTitles()
})
</script>

<template>
  <div class="my-subs">
    <header class="my-subs__header">
      <h1 class="my-subs__title">我的订阅</h1>
      <p class="my-subs__subtitle">管理你订阅的待办事项和关注的标签</p>
    </header>

    <!-- Tabs -->
    <div class="my-subs__tabs">
      <button
        class="filter-tab"
        :class="{ 'filter-tab--active': activeTab === 'todos' }"
        @click="activeTab = 'todos'"
      >
        TODO 订阅
        <span class="filter-tab__count">{{ todos.length }}</span>
      </button>
      <button
        class="filter-tab"
        :class="{ 'filter-tab--active': activeTab === 'tags' }"
        @click="activeTab = 'tags'"
      >
        关注标签
        <span class="filter-tab__count">{{ tags.length }}</span>
      </button>
    </div>

    <!-- Loading -->
    <div v-if="loading && todos.length === 0 && tags.length === 0" class="my-subs__loading">
      <n-spin size="large" />
    </div>

    <!-- TODO Tab -->
    <template v-else-if="activeTab === 'todos'">
      <div v-if="todos.length === 0" class="my-subs__empty">
        <n-empty description="暂无订阅的 TODO 项" />
      </div>

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
                <span class="todo-card__title" :class="{ 'todo-card__title--done': todo.completed }">
                  {{ todo.title }}
                </span>
              </div>
              <n-badge :value="todo.subscriber_count" :max="99" class="todo-card__badge">
                <span class="todo-card__badge-label">订阅</span>
              </n-badge>
            </div>
          </div>
        </section>
      </div>
    </template>

    <!-- Tags Tab -->
    <template v-else>
      <div v-if="tags.length === 0" class="my-subs__empty">
        <n-empty description="暂无关注的标签" />
      </div>

      <div v-else class="tags-grid">
        <div
          v-for="tag in tags"
          :key="tag.id"
          class="tag-card"
        >
          <n-tag :bordered="false" type="info" size="large">
            {{ tag.name }}
          </n-tag>
          <span class="tag-card__slug">#{{ tag.slug }}</span>
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.my-subs {
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
.my-subs__header {
  margin-bottom: var(--space-8);
  padding-bottom: var(--space-6);
  border-bottom: 1px solid var(--color-border);
}

.my-subs__title {
  font-family: var(--font-display);
  font-size: var(--text-4xl);
  font-weight: 700;
  color: var(--color-text-primary);
  letter-spacing: -0.03em;
  margin-bottom: var(--space-2);
}

.my-subs__subtitle {
  font-size: var(--text-lg);
  color: var(--color-text-tertiary);
  font-weight: 300;
}

/* --- Tabs --- */
.my-subs__tabs {
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
.my-subs__loading,
.my-subs__empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 300px;
  gap: var(--space-4);
}

/* --- Todo Groups --- */
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
  padding: var(--space-4);
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

/* --- Tags Grid --- */
.tags-grid {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-3);
}

.tag-card {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-3) var(--space-4);
  background: var(--color-bg-elevated);
  border: 1px solid var(--color-border-light);
  border-radius: var(--radius-md);
  transition: all var(--transition-fast);
}

.tag-card:hover {
  border-color: var(--color-accent);
  box-shadow: var(--shadow-sm);
}

.tag-card__slug {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  font-family: var(--font-mono);
}

/* --- Responsive --- */
@media (max-width: 767px) {
  .my-subs__title {
    font-size: var(--text-3xl);
  }

  .my-subs__tabs {
    flex-wrap: wrap;
  }

  .todo-card {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-3);
    padding: var(--space-4) 0;
  }
}
</style>
