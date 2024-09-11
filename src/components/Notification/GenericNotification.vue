<template>
    <div 
      class="notification" 
      :style="{ borderColor: borderColor }"
      @click.self="close"
      @mouseenter="$emit('mouseenter')"
      @mouseleave="$emit('mouseleave')"
    >
      <div class="notification-icon">
        <component :is="icon" :size="24" />
      </div>
      <div class="notification-content">
        <h3 class="notification-header">{{ header }}</h3>
        <p class="notification-message">{{ message }}</p>
      </div>
    </div>
  </template>
  
  <script>
  import * as LucideIcons from 'lucide-vue-next'
  
  export default {
    components: {
      ...LucideIcons
    },
    props: {
      id: {
        type: Number,
        required: true
      },
      icon: {
        type: String,
        default: 'InfoIcon'
      },
      header: {
        type: String,
        required: true
      },
      message: {
        type: String,
        required: true
      },
      borderColor: {
        type: String,
        default: '#007bff'
      }
    },
    methods: {
      close() {
        this.$emit('close', this.id)
      }
    }
  }
  </script>
  
  <style scoped>
  .notification {
    width: 300px;
    background-color: #2a2b3d;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    border-radius: 4px;
    border-left: 4px solid;
    display: flex;
    align-items: start;
    padding: 12px;
    margin-bottom: 10px;
    cursor: pointer;
  }
  
  .notification-icon {
    margin-right: 12px;
    color: #3498db;
  }
  
  .notification-content {
    flex-grow: 1;
  }
  
  .notification-header {
    margin: 0 0 4px 0;
    font-size: 16px;
  }
  
  .notification-message {
    margin: 0;
    font-size: 14px;
    color: var(--text-color);
  }
  </style>