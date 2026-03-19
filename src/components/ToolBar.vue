<template>
  <div class="tool-bar">
    <div class="tool-group">
      <button @click="$emit('open-file')" title="打开文件">📂</button>
      <button title="保存">💾</button>
      <button title="打印">🖨️</button>
    </div>
    
    <div class="separator"></div>
    
    <div class="tool-group">
      <button @click="goToFirst" :disabled="currentPage <= 1" title="首页">⏮️</button>
      <button @click="goToPrevious" :disabled="currentPage <= 1" title="上一页">⬅️</button>
      <input 
        type="number" 
        :value="currentPage" 
        @change="handlePageInput"
        :min="1" 
        :max="totalPages"
        class="page-input"
      >
      <span class="page-info">/{{ totalPages }}</span>
      <button @click="goToNext" :disabled="currentPage >= totalPages" title="下一页">➡️</button>
      <button @click="goToLast" :disabled="currentPage >= totalPages" title="末页">⏭️</button>
    </div>
    
    <div class="separator"></div>
    
    <div class="tool-group">
      <button @click="zoomOut" title="缩小">🔍-</button>
      <span class="scale-info">{{ Math.round(scale * 100) }}%</span>
      <button @click="zoomIn" title="放大">🔍+</button>
      <button @click="fitWidth" title="适合宽度">↔️</button>
      <button @click="fitPage" title="适合页面">⬜</button>
      <button @click="actualSize" title="实际大小">1:1</button>
    </div>
    
    <div class="separator"></div>
    
    <div class="tool-group">
      <button @click="rotateRight" title="顺时针旋转">↻</button>
      <button @click="rotateLeft" title="逆时针旋转">↺</button>
    </div>
    
    <div class="separator"></div>
    
    <div class="tool-group">
      <button 
        :class="{ active: viewMode === 'single' }"
        @click="$emit('view-mode', 'single')" 
        title="单页模式"
      >📄</button>
      <button 
        :class="{ active: viewMode === 'continuous' }"
        @click="$emit('view-mode', 'continuous')" 
        title="连续滚动"
      >📜</button>
    </div>
    
    <div class="separator"></div>
    
    <div class="tool-group signature">
      <div 
        class="signature-indicator"
        :class="signatureClass"
        title="签名状态"
      ></div>
      <button title="验证详情">ℹ️</button>
    </div>
    
    <div class="separator"></div>
    
    <div class="tool-group">
      <button :class="{ active: currentTool === 'hand' }" @click="setTool('hand')" title="手形工具">✋</button>
      <button :class="{ active: currentTool === 'select' }" @click="setTool('select')" title="文本选择">📝</button>
      <button title="快照">📷</button>
      <button title="查找">🔍</button>
    </div>
  </div>
</template>

<script>
export default {
  name: 'ToolBar',
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
    viewMode: {
      type: String,
      default: 'single'
    },
    signatures: {
      type: Array,
      default: () => []
    }
  },
  data() {
    return {
      currentTool: 'hand'
    }
  },
  computed: {
    signatureClass() {
      if (this.signatures.length === 0) return 'gray'
      const valid = this.signatures.every(s => s.valid)
      const warning = this.signatures.some(s => s.warning)
      if (valid && !warning) return 'green'
      if (warning) return 'yellow'
      return 'red'
    }
  },
  methods: {
    goToFirst() {
      this.$emit('page-change', 1)
    },
    goToPrevious() {
      this.$emit('page-change', this.currentPage - 1)
    },
    goToNext() {
      this.$emit('page-change', this.currentPage + 1)
    },
    goToLast() {
      this.$emit('page-change', this.totalPages)
    },
    handlePageInput(event) {
      let page = parseInt(event.target.value)
      if (page >= 1 && page <= this.totalPages) {
        this.$emit('page-change', page)
      } else {
        event.target.value = this.currentPage
      }
    },
    zoomIn() {
      this.$emit('zoom', Math.min(this.scale + 0.1, 3.0))
    },
    zoomOut() {
      this.$emit('zoom', Math.max(this.scale - 0.1, 0.25))
    },
    fitWidth() {
      this.$emit('zoom', 1.5)
    },
    fitPage() {
      this.$emit('zoom', 0.8)
    },
    actualSize() {
      this.$emit('zoom', 1.0)
    },
    rotateRight() {
      this.$emit('rotate', 90)
    },
    rotateLeft() {
      this.$emit('rotate', -90)
    },
    setTool(tool) {
      this.currentTool = tool
    }
  }
}
</script>

<style scoped>
.tool-bar {
  height: 36px;
  background-color: #fafafa;
  border-bottom: 1px solid #ddd;
  display: flex;
  align-items: center;
  padding: 0 8px;
  gap: 4px;
}

.tool-group {
  display: flex;
  align-items: center;
  gap: 4px;
}

.separator {
  width: 1px;
  height: 24px;
  background-color: #ddd;
  margin: 0 8px;
}

button {
  padding: 6px 8px;
  font-size: 14px;
}

button.active {
  background-color: #4a90d9;
  color: white;
  border-color: #3a80c9;
}

.page-input {
  width: 50px;
  text-align: center;
}

.page-info {
  font-size: 13px;
  color: #666;
}

.scale-info {
  font-size: 13px;
  color: #666;
  min-width: 45px;
  text-align: center;
}

.signature-indicator {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  border: 1px solid #999;
}

.signature-indicator.green {
  background-color: #4caf50;
  border-color: #388e3c;
}

.signature-indicator.yellow {
  background-color: #ffc107;
  border-color: #ffa000;
}

.signature-indicator.red {
  background-color: #f44336;
  border-color: #d32f2f;
}

.signature-indicator.gray {
  background-color: #9e9e9e;
  border-color: #757575;
}
</style>
