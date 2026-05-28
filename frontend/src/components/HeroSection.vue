<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { NButton } from 'naive-ui'

const phrases = [
  '思考、记录与分享',
  '探索技术的边界',
  '用代码改变世界',
  '与志同道合的人交流',
]

const currentIndex = ref(0)
let timer: ReturnType<typeof setInterval> | null = null

function startCycle() {
  timer = setInterval(() => {
    currentIndex.value = (currentIndex.value + 1) % phrases.length
  }, 3000)
}

function scrollToArticles() {
  const target = document.querySelector('.home__search')
  if (target) {
    target.scrollIntoView({ behavior: 'smooth', block: 'start' })
  }
}

onMounted(() => {
  startCycle()
})

onUnmounted(() => {
  if (timer) clearInterval(timer)
})
</script>

<template>
  <section class="hero">
    <div class="hero__content">
      <h1 class="hero__title">ZYBlog</h1>

      <div class="hero__phrase-wrapper">
        <Transition name="phrase" mode="out-in">
          <p :key="currentIndex" class="hero__phrase">
            {{ phrases[currentIndex] }}
          </p>
        </Transition>
      </div>

      <n-button
        class="hero__cta"
        size="large"
        @click="scrollToArticles"
      >
        开始阅读
      </n-button>
    </div>
  </section>
</template>

<style scoped>
.hero {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 60vh;
  padding: var(--space-16) var(--space-6);
  text-align: center;
}

.hero__content {
  max-width: var(--content-max-width);
}

/* --- Headline --- */
.hero__title {
  font-family: var(--font-display);
  font-size: 4.5rem;
  font-weight: 700;
  color: var(--color-text-primary);
  letter-spacing: -0.04em;
  line-height: 1.1;
  margin-bottom: var(--space-6);
}

/* --- Animated phrase --- */
.hero__phrase-wrapper {
  min-height: 2.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: var(--space-10);
}

.hero__phrase {
  font-family: var(--font-body);
  font-size: var(--text-2xl);
  font-weight: 300;
  line-height: 1.4;
  background: linear-gradient(
    135deg,
    var(--color-accent) 0%,
    var(--color-accent-hover) 100%
  );
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  letter-spacing: 0.02em;
}

/* --- Vue transition: fade + slide --- */
.phrase-enter-active,
.phrase-leave-active {
  transition: opacity 0.5s ease, transform 0.5s ease;
}

.phrase-enter-from {
  opacity: 0;
  transform: translateY(10px);
}

.phrase-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}

/* --- CTA button --- */
.hero__cta {
  font-family: var(--font-body);
  font-weight: 500;
  letter-spacing: 0.02em;
}

/* --- Responsive --- */
@media (max-width: 767px) {
  .hero {
    min-height: 50vh;
    padding: var(--space-12) var(--space-4);
  }

  .hero__title {
    font-size: var(--text-4xl);
  }

  .hero__phrase {
    font-size: var(--text-xl);
  }
}
</style>
