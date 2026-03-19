<template>
  <div class="menu-bar">
    <div class="menu-item" @click="handleOpenFile">
      文件
    </div>
    <div class="menu-item">
      编辑
    </div>
    <div class="menu-item">
      视图
    </div>
    <div class="menu-item">
      导航
    </div>
    <div class="menu-item">
      工具
    </div>
    <div class="menu-item">
      帮助
    </div>
  </div>
</template>

<script>
export default {
  name: 'MenuBar',
  methods: {
    handleOpenFile() {
      this.$emit('open-file')
    }
  }
}
</script>

<style scoped>
.menu-bar {
  height: 28px;
  background-color: #f5f5f5;
  border-bottom: 1px solid #ddd;
  display: flex;
  align-items: center;
  padding: 0 8px;
}

.menu-item {
  padding: 4px 12px;
  cursor: pointer;
  font-size: 13px;
  border-radius: 4px;
  transition: background-color 0.2s;
}

.menu-item:hover {
  background-color: #e0e0e0;
}
</style>
