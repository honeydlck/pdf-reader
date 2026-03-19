<template>
  <div class="status-bar">
    <div class="status-left">
      <span class="status-item">页 {{ currentPage }}/{{ totalPages }}</span>
      <span class="status-item">{{ Math.round(scale * 100) }}%</span>
      <span v-if="fileSize > 0" class="status-item">{{ formatFileSize(fileSize) }}</span>
    </div>
    
    <div class="status-center">
      <span class="status-item signature-status" :class="signatureStatusClass">
        {{ signatureStatusText }}
      </span>
    </div>
    
    <div class="status-right">
      <span class="status-item">就绪</span>
    </div>
  </div>
</template>

<script>
export default {
  name: 'StatusBar',
  props: {
    currentPage: {
      type: Number,
      default: 1
    },
    totalPages: {
      type: Number,
      default: 0
    },
    scale: {
      type: Number,
      default: 1.0
    },
    signatures: {
      type: Array,
      default: () => []
    },
    fileSize: {
      type: Number,
      default: 0
    }
  },
  computed: {
    signatureStatusClass() {
      if (this.signatures.length === 0) return 'gray'
      const valid = this.signatures.every(s => s.valid)
      const warning = this.signatures.some(s => s.warning)
      if (valid && !warning) return 'valid'
      if (warning) return 'warning'
      return 'invalid'
    },
    signatureStatusText() {
      if (this.signatures.length === 0) return '无签名'
      const valid = this.signatures.every(s => s.valid)
      const warning = this.signatures.some(s => s.warning)
      if (valid && !warning) return '签名有效'
      if (warning) return '签名警告'
      return '签名无效'
    }
  },
  methods: {
    formatFileSize(bytes) {
      if (bytes === 0) return '0 B'
      const k = 1024
      const sizes = ['B', 'KB', 'MB', 'GB']
      const i = Math.floor(Math.log(bytes) / Math.log(k))
      return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
    }
  }
}
</script>

<style scoped>
.status-bar {
  height: 24px;
  background-color: #f5f5f5;
  border-top: 1px solid #ddd;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 12px;
  font-size: 12px;
  color: #666;
}

.status-left,
.status-center,
.status-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.status-item {
  white-space: nowrap;
}

.signature-status {
  padding: 2px 8px;
  border-radius: 4px;
  font-weight: 500;
}

.signature-status.gray {
  background-color: #e0e0e0;
  color: #666;
}

.signature-status.valid {
  background-color: #c8e6c9;
  color: #2e7d32;
}

.signature-status.warning {
  background-color: #fff9c4;
  color: #f57f17;
}

.signature-status.invalid {
  background-color: #ffcdd2;
  color: #c62828;
}
</style>
