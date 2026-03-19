<template>
  <div class="title-bar">
    <div class="title-content">
      <span class="app-name">PDF阅读器</span>
      <span v-if="fileName" class="separator">-</span>
      <span v-if="fileName" class="file-name">{{ fileName }}</span>
    </div>
  </div>
</template>

<script>
export default {
  name: 'TitleBar',
  props: {
    fileName: {
      type: String,
      default: ''
    }
  }
}
</script>

<style scoped>
.title-bar {
  height: 32px;
  background: linear-gradient(180deg, #3a3a3a 0%, #2a2a2a 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  -webkit-app-region: drag;
}

.title-content {
  display: flex;
  align-items: center;
  gap: 8px;
}

.app-name {
  font-weight: 500;
}

.separator {
  opacity: 0.6;
}

.file-name {
  opacity: 0.9;
  max-width: 400px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
