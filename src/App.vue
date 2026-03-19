<template>
  <div class="app-container">
    <TitleBar :fileName="currentFileName" />
    <MenuBar @open-file="handleOpenFile" />
    <ToolBar 
      :currentPage="currentPage"
      :totalPages="totalPages"
      :scale="scale"
      :signatures="signatures"
      @open-file="handleOpenFile"
      @page-change="handlePageChange"
      @zoom="handleZoom"
      @rotate="handleRotate"
      @view-mode="handleViewMode"
    />
    <div class="main-content">
      <NavigationPanel 
        :pages="pdfPages"
        :signatures="signatures"
        :activeTab="activeNavTab"
        @navigate="handleNavigate"
        @tab-change="handleNavTabChange"
      />
      <PDFViewer 
        ref="pdfViewer"
        :pdfData="pdfData"
        :currentPage="currentPage"
        :scale="scale"
        :rotation="rotation"
        :viewMode="viewMode"
        @open-file="handleOpenFile"
        @page-rendered="handlePageRendered"
        @page-count="handlePageCount"
      />
    </div>
    <StatusBar 
      :currentPage="currentPage"
      :totalPages="totalPages"
      :scale="scale"
      :signatures="signatures"
      :fileSize="fileSize"
    />
  </div>
</template>

<script>
import TitleBar from './components/TitleBar.vue'
import MenuBar from './components/MenuBar.vue'
import ToolBar from './components/ToolBar.vue'
import NavigationPanel from './components/NavigationPanel.vue'
import PDFViewer from './components/PDFViewer.vue'
import StatusBar from './components/StatusBar.vue'

let open, readBinaryFile
try {
  open = require('@tauri-apps/plugin-dialog').open
  readBinaryFile = require('@tauri-apps/plugin-fs').readBinaryFile
} catch (e) {
  console.log('Tauri not available, using browser fallback')
}

export default {
  name: 'App',
  components: {
    TitleBar,
    MenuBar,
    ToolBar,
    NavigationPanel,
    PDFViewer,
    StatusBar
  },
  data() {
    return {
      pdfData: null,
      currentFileName: '',
      currentPage: 1,
      totalPages: 0,
      scale: 1.0,
      rotation: 0,
      viewMode: 'single',
      activeNavTab: 'thumbnails',
      signatures: [],
      pdfPages: [],
      fileSize: 0
    }
  },
  methods: {
    handleOpenFile() {
      if (open && readBinaryFile) {
        this.handleOpenFileTauri()
      } else {
        this.handleOpenFileBrowser()
      }
    },
    async handleOpenFileTauri() {
      try {
        const selected = await open({
          multiple: false,
          filters: [
            { name: 'PDF Files', extensions: ['pdf'] }
          ]
        })
        if (selected) {
          await this.loadPDFFromPath(selected)
        }
      } catch (error) {
        console.error('Error opening file with Tauri:', error)
        this.handleOpenFileBrowser()
      }
    },
    handleOpenFileBrowser() {
      const input = document.createElement('input')
      input.type = 'file'
      input.accept = '.pdf'
      input.onchange = (e) => {
        const file = e.target.files[0]
        if (file) {
          this.loadPDFFromFile(file)
        }
      }
      input.click()
    },
    async loadPDFFromPath(filePath) {
      try {
        const binaryData = await readBinaryFile(filePath)
        this.pdfData = new Uint8Array(binaryData)
        this.currentFileName = filePath.split('/').pop()
        this.fileSize = binaryData.length
        this.currentPage = 1
      } catch (error) {
        console.error('Error loading PDF from path:', error)
      }
    },
    loadPDFFromFile(file) {
      const reader = new FileReader()
      reader.onload = (e) => {
        const arrayBuffer = e.target.result
        this.pdfData = new Uint8Array(arrayBuffer)
        this.currentFileName = file.name
        this.fileSize = file.size
        this.currentPage = 1
      }
      reader.readAsArrayBuffer(file)
    },
    handlePageChange(page) {
      this.currentPage = page
    },
    handleZoom(scale) {
      if (typeof scale === 'number') {
        this.scale = scale
      }
    },
    handleRotate(rotation) {
      this.rotation = (this.rotation + rotation + 360) % 360
    },
    handleViewMode(mode) {
      this.viewMode = mode
    },
    handleNavigate(page) {
      this.currentPage = page
    },
    handleNavTabChange(tab) {
      this.activeNavTab = tab
    },
    handlePageCount(count) {
      this.totalPages = count
      this.pdfPages = Array.from({ length: count }, (_, i) => i + 1)
    },
    handlePageRendered(pageNum) {
    }
  }
}
</script>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100%;
  overflow: hidden;
}

.main-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}
</style>
