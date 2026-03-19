<template>
  <div class="pdf-viewer">
    <div v-if="!pdfData" class="empty-viewer">
      <div class="empty-content">
        <div class="empty-icon">📄</div>
        <div class="empty-text">请打开一个PDF文件</div>
        <button @click="$emit('open-file')" class="open-button">打开文件</button>
      </div>
    </div>
    <div v-else class="pdf-container" ref="pdfContainer">
      <div 
        v-for="pageNum in pagesToRender" 
        :key="pageNum"
        class="page-wrapper"
        :style="pageWrapperStyle"
      >
        <canvas 
          :ref="el => setPageRef(el, pageNum)"
          class="pdf-page"
        ></canvas>
        <div class="page-number-label">页 {{ pageNum }}</div>
      </div>
    </div>
  </div>
</template>

<script>
import * as pdfjsLib from 'pdfjs-dist'

pdfjsLib.GlobalWorkerOptions.workerSrc = `https://cdnjs.cloudflare.com/ajax/libs/pdf.js/${pdfjsLib.version}/pdf.worker.min.js`

export default {
  name: 'PDFViewer',
  props: {
    pdfData: {
      type: Uint8Array,
      default: null
    },
    currentPage: {
      type: Number,
      default: 1
    },
    scale: {
      type: Number,
      default: 1.0
    },
    rotation: {
      type: Number,
      default: 0
    },
    viewMode: {
      type: String,
      default: 'single'
    }
  },
  data() {
    return {
      pdfDoc: null,
      pageRefs: {},
      totalPages: 0,
      pagesToRender: []
    }
  },
  computed: {
    pageWrapperStyle() {
      return {
        transform: `rotate(${this.rotation}deg)`
      }
    }
  },
  watch: {
    pdfData: {
      immediate: true,
      handler(newVal) {
        if (newVal) {
          this.loadPDF()
        } else {
          this.pdfDoc = null
          this.totalPages = 0
        }
      }
    },
    currentPage: {
      handler() {
        this.updatePagesToRender()
      }
    },
    viewMode: {
      handler() {
        this.updatePagesToRender()
      }
    },
    scale: {
      handler() {
        this.renderPages()
      }
    },
    rotation: {
      handler() {
        this.renderPages()
      }
    }
  },
  methods: {
    setPageRef(el, pageNum) {
      if (el) {
        this.pageRefs[pageNum] = el
      }
    },
    async loadPDF() {
      try {
        const loadingTask = pdfjsLib.getDocument({ data: this.pdfData })
        this.pdfDoc = await loadingTask.promise
        this.totalPages = this.pdfDoc.numPages
        this.$emit('page-count', this.totalPages)
        this.updatePagesToRender()
        this.$nextTick(() => {
          this.renderPages()
        })
      } catch (error) {
        console.error('Error loading PDF:', error)
      }
    },
    updatePagesToRender() {
      if (this.viewMode === 'single') {
        this.pagesToRender = [this.currentPage]
      } else {
        this.pagesToRender = Array.from({ length: this.totalPages }, (_, i) => i + 1)
      }
      this.$nextTick(() => {
        this.renderPages()
      })
    },
    async renderPages() {
      for (const pageNum of this.pagesToRender) {
        await this.renderPage(pageNum)
      }
    },
    async renderPage(pageNum) {
      if (!this.pdfDoc || !this.pageRefs[pageNum]) return
      
      try {
        const page = await this.pdfDoc.getPage(pageNum)
        const canvas = this.pageRefs[pageNum]
        const context = canvas.getContext('2d')
        
        const viewport = page.getViewport({ 
          scale: this.scale,
          rotation: this.rotation
        })
        
        canvas.height = viewport.height
        canvas.width = viewport.width
        
        const renderContext = {
          canvasContext: context,
          viewport: viewport
        }
        
        await page.render(renderContext).promise
        this.$emit('page-rendered', pageNum)
      } catch (error) {
        console.error('Error rendering page:', error)
      }
    }
  }
}
</script>

<style scoped>
.pdf-viewer {
  flex: 1;
  overflow: auto;
  background-color: #525659;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  padding: 20px;
}

.empty-viewer {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.empty-content {
  text-align: center;
  color: #999;
}

.empty-icon {
  font-size: 64px;
  margin-bottom: 16px;
}

.empty-text {
  font-size: 18px;
  margin-bottom: 24px;
}

.open-button {
  padding: 10px 24px;
  font-size: 16px;
}

.pdf-container {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.page-wrapper {
  position: relative;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.pdf-page {
  display: block;
  background-color: white;
}

.page-number-label {
  position: absolute;
  bottom: -24px;
  left: 50%;
  transform: translateX(-50%);
  color: #ccc;
  font-size: 12px;
}
</style>
