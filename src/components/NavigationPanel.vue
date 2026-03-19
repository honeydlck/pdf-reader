<template>
  <div class="navigation-panel">
    <div class="nav-tabs">
      <button 
        :class="{ active: activeTab === 'thumbnails' }"
        @click="$emit('tab-change', 'thumbnails')"
        title="缩略图"
      >🖼️</button>
      <button 
        :class="{ active: activeTab === 'bookmarks' }"
        @click="$emit('tab-change', 'bookmarks')"
        title="书签"
      >📑</button>
      <button 
        :class="{ active: activeTab === 'signatures' }"
        @click="$emit('tab-change', 'signatures')"
        title="签名"
      >✒️</button>
    </div>
    
    <div class="nav-content">
      <div v-if="activeTab === 'thumbnails'" class="thumbnail-list">
        <div 
          v-for="page in pages" 
          :key="page"
          class="thumbnail-item"
          @click="$emit('navigate', page)"
        >
          <div class="thumbnail-placeholder">
            <span class="page-number">{{ page }}</span>
          </div>
          <span class="thumbnail-label">页 {{ page }}</span>
        </div>
      </div>
      
      <div v-else-if="activeTab === 'bookmarks'" class="bookmarks-list">
        <div class="empty-state">
          暂无书签
        </div>
      </div>
      
      <div v-else-if="activeTab === 'signatures'" class="signatures-list">
        <div v-if="signatures.length === 0" class="empty-state">
          暂无签名
        </div>
        <div v-else>
          <div 
            v-for="(signature, index) in signatures" 
            :key="index"
            class="signature-item"
          >
            <div 
              class="signature-status"
              :class="signature.valid ? 'valid' : 'invalid'"
            ></div>
            <div class="signature-info">
              <div class="signature-name">{{ signature.name || '未知签名者' }}</div>
              <div class="signature-date">{{ signature.date || '未知时间' }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'NavigationPanel',
  props: {
    pages: {
      type: Array,
      default: () => []
    },
    signatures: {
      type: Array,
      default: () => []
    },
    activeTab: {
      type: String,
      default: 'thumbnails'
    }
  }
}
</script>

<style scoped>
.navigation-panel {
  width: 200px;
  background-color: #f5f5f5;
  border-right: 1px solid #ddd;
  display: flex;
  flex-direction: column;
}

.nav-tabs {
  display: flex;
  padding: 4px;
  gap: 4px;
  border-bottom: 1px solid #ddd;
}

.nav-tabs button {
  flex: 1;
  padding: 6px;
  font-size: 16px;
}

.nav-tabs button.active {
  background-color: #4a90d9;
  color: white;
  border-color: #3a80c9;
}

.nav-content {
  flex: 1;
  overflow-y: auto;
}

.thumbnail-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 8px;
}

.thumbnail-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  transition: background-color 0.2s;
}

.thumbnail-item:hover {
  background-color: #e0e0e0;
}

.thumbnail-placeholder {
  width: 120px;
  height: 160px;
  background-color: #fff;
  border: 1px solid #ddd;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.page-number {
  font-size: 24px;
  color: #999;
}

.thumbnail-label {
  font-size: 12px;
  color: #666;
}

.empty-state {
  padding: 24px;
  text-align: center;
  color: #999;
}

.signatures-list {
  padding: 8px;
}

.signature-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px;
  background-color: #fff;
  border: 1px solid #ddd;
  border-radius: 4px;
  margin-bottom: 8px;
}

.signature-status {
  width: 12px;
  height: 12px;
  border-radius: 50%;
}

.signature-status.valid {
  background-color: #4caf50;
}

.signature-status.invalid {
  background-color: #f44336;
}

.signature-info {
  flex: 1;
}

.signature-name {
  font-size: 13px;
  font-weight: 500;
}

.signature-date {
  font-size: 11px;
  color: #999;
}
</style>
