<script setup lang="ts">
import { onMounted, ref } from 'vue'
import {
  NSpin,
  NEmpty,
  NButton,
  NInput,
  NPopconfirm,
  NSpace,
  useMessage,
} from 'naive-ui'
import { useTags } from '../composables/useTags'

const message = useMessage()
const { tags, loading, fetchTags, createTag, deleteTag } = useTags()

const newTagName = ref('')
const creating = ref(false)

function formatDate(dateStr: string): string {
  return new Date(dateStr).toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  })
}

async function handleCreate() {
  const name = newTagName.value.trim()
  if (!name) {
    message.warning('请输入标签名称')
    return
  }

  creating.value = true
  const result = await createTag(name)
  creating.value = false

  if (result.success) {
    message.success('标签创建成功')
    newTagName.value = ''
  } else {
    message.error(result.error || '创建失败')
  }
}

async function handleDelete(tagId: number) {
  const result = await deleteTag(tagId)
  if (result.success) {
    message.success('标签已删除')
  } else {
    message.error(result.error || '删除失败')
  }
}

onMounted(() => {
  fetchTags()
})
</script>

<template>
  <div class="tags-admin">
    <header class="tags-admin__header">
      <h1 class="tags-admin__title">标签管理</h1>
      <p class="tags-admin__subtitle">创建和管理文章标签</p>
    </header>

    <!-- Create Tag Form -->
    <div class="tags-admin__create">
      <n-space :size="12" align="center">
        <n-input
          v-model:value="newTagName"
          placeholder="输入新标签名称"
          :disabled="creating"
          style="max-width: 280px;"
          @keyup.enter="handleCreate"
        />
        <n-button
          type="primary"
          :loading="creating"
          :disabled="!newTagName.trim()"
          @click="handleCreate"
        >
          创建标签
        </n-button>
      </n-space>
    </div>

    <!-- Loading -->
    <div v-if="loading && tags.length === 0" class="tags-admin__loading">
      <n-spin size="large" />
    </div>

    <!-- Empty -->
    <div v-else-if="tags.length === 0" class="tags-admin__empty">
      <n-empty description="暂无标签，请创建第一个标签" />
    </div>

    <!-- Tags List -->
    <div v-else class="tag-list">
      <div
        v-for="tag in tags"
        :key="tag.id"
        class="tag-card"
      >
        <div class="tag-card__body">
          <span class="tag-card__name">{{ tag.name }}</span>
          <span class="tag-card__slug">{{ tag.slug }}</span>
          <time class="tag-card__date" :datetime="tag.created_at">
            {{ formatDate(tag.created_at) }}
          </time>
        </div>

        <div class="tag-card__actions">
          <n-popconfirm @positive-click="handleDelete(tag.id)">
            <template #trigger>
              <n-button type="error" size="small" quaternary>
                删除
              </n-button>
            </template>
            确定删除标签「{{ tag.name }}」？
          </n-popconfirm>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.tags-admin {
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

.tags-admin__header {
  margin-bottom: var(--space-10);
  padding-bottom: var(--space-6);
  border-bottom: 1px solid var(--color-border);
}

.tags-admin__title {
  font-family: var(--font-display);
  font-size: var(--text-4xl);
  font-weight: 700;
  color: var(--color-text-primary);
  letter-spacing: -0.03em;
  margin-bottom: var(--space-2);
}

.tags-admin__subtitle {
  font-size: var(--text-lg);
  color: var(--color-text-tertiary);
  font-weight: 300;
}

.tags-admin__create {
  margin-bottom: var(--space-8);
}

.tags-admin__loading,
.tags-admin__empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 300px;
  gap: var(--space-4);
}

.tag-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.tag-card {
  display: flex;
  align-items: center;
  gap: var(--space-6);
  padding: var(--space-4) var(--space-4);
  border-bottom: 1px solid var(--color-border-light);
  border-radius: var(--radius-md);
  transition: background-color var(--transition-fast);
}

.tag-card:hover {
  background-color: var(--color-bg-sunken);
}

.tag-card__body {
  flex: 1;
  min-width: 0;
  display: flex;
  align-items: center;
  gap: var(--space-4);
  flex-wrap: wrap;
}

.tag-card__name {
  font-weight: 600;
  font-size: var(--text-base);
  color: var(--color-text-primary);
}

.tag-card__slug {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  font-family: var(--font-mono);
  background: var(--color-bg-sunken);
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-sm);
}

.tag-card__date {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  font-variant-numeric: tabular-nums;
}

.tag-card__actions {
  flex-shrink: 0;
}

@media (max-width: 767px) {
  .tags-admin__title {
    font-size: var(--text-3xl);
  }

  .tag-card {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-3);
    padding: var(--space-4) 0;
  }

  .tag-card__body {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-2);
  }

  .tag-card__actions {
    width: 100%;
  }
}
</style>
