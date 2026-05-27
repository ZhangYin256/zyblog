<script setup lang="ts">
import { ref, computed } from 'vue'

interface Props {
  /** 视频源 URL */
  src: string
  /** 视频 MIME 类型（可选，默认根据扩展名推断） */
  type?: string
  /** 是否自动播放 */
  autoplay?: boolean
  /** 是否循环播放 */
  loop?: boolean
  /** 是否静音 */
  muted?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  type: '',
  autoplay: false,
  loop: false,
  muted: false,
})

const videoRef = ref<HTMLVideoElement | null>(null)
const isPlaying = ref(false)
const hasError = ref(false)
const showControls = ref(true)

/** 根据文件扩展名推断 MIME 类型 */
const videoType = computed(() => {
  if (props.type) return props.type
  const ext = props.src.split('.').pop()?.toLowerCase()
  const mimeMap: Record<string, string> = {
    mp4: 'video/mp4',
    webm: 'video/webm',
    ogg: 'video/ogg',
    ogv: 'video/ogg',
  }
  return mimeMap[ext || ''] || 'video/mp4'
})

function togglePlay() {
  const video = videoRef.value
  if (!video) return

  if (video.paused) {
    video.play()
    isPlaying.value = true
  } else {
    video.pause()
    isPlaying.value = false
  }
}

function handleEnded() {
  isPlaying.value = false
}

function handleError() {
  hasError.value = true
}

function handlePlay() {
  isPlaying.value = true
}

function handlePause() {
  isPlaying.value = false
}
</script>

<template>
  <div
    class="video-player"
    :class="{ 'video-player--playing': isPlaying }"
    @mouseenter="showControls = true"
    @mouseleave="showControls = isPlaying"
  >
    <!-- Error State -->
    <div v-if="hasError" class="video-player__error">
      <span class="video-player__error-icon">⚠</span>
      <span class="video-player__error-text">视频加载失败</span>
    </div>

    <!-- Video Element -->
    <video
      v-else
      ref="videoRef"
      class="video-player__video"
      :src="src"
      :type="videoType"
      :autoplay="autoplay"
      :loop="loop"
      :muted="muted"
      preload="metadata"
      playsinline
      @ended="handleEnded"
      @error="handleError"
      @play="handlePlay"
      @pause="handlePause"
      @click="togglePlay"
    />

    <!-- Play/Pause Overlay -->
    <div
      v-if="!hasError"
      class="video-player__overlay"
      :class="{ 'video-player__overlay--visible': !isPlaying }"
      @click="togglePlay"
    >
      <span class="video-player__play-btn">
        {{ isPlaying ? '❚❚' : '▶' }}
      </span>
    </div>
  </div>
</template>

<style scoped>
.video-player {
  position: relative;
  width: 100%;
  border-radius: var(--radius-lg);
  overflow: hidden;
  background: var(--color-bg-sunken);
  border: 1px solid var(--color-border-light);
  cursor: pointer;
  transition: box-shadow var(--transition-base);
}

.video-player:hover {
  box-shadow: var(--shadow-md);
}

.video-player__video {
  display: block;
  width: 100%;
  height: auto;
  max-height: 520px;
  object-fit: contain;
  background: #000;
}

/* --- 播放/暂停遮罩 --- */
.video-player__overlay {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.25);
  opacity: 0;
  transition: opacity var(--transition-base);
  pointer-events: none;
}

.video-player__overlay--visible {
  opacity: 1;
  pointer-events: auto;
}

.video-player__play-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 64px;
  height: 64px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.92);
  color: var(--color-text-primary);
  font-size: 24px;
  box-shadow: var(--shadow-lg);
  transition: transform var(--transition-fast), background var(--transition-fast);
  letter-spacing: -2px;
  padding-left: 2px;
}

.video-player:hover .video-player__play-btn {
  transform: scale(1.08);
  background: #fff;
}

/* --- 错误状态 --- */
.video-player__error {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  min-height: 200px;
  color: var(--color-text-tertiary);
}

.video-player__error-icon {
  font-size: var(--text-3xl);
  opacity: 0.5;
}

.video-player__error-text {
  font-size: var(--text-sm);
  font-family: var(--font-body);
}

/* --- 响应式 --- */
@media (max-width: 767px) {
  .video-player__play-btn {
    width: 48px;
    height: 48px;
    font-size: 18px;
  }

  .video-player__video {
    max-height: 300px;
  }
}
</style>
