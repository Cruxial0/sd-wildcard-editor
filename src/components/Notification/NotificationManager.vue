<template>
    <div class="notification-container">
      <transition-group name="notification-list">
        <GenericNotification
          v-for="notification in notifications"
          :key="notification.id"
          v-bind="notification"
          @close="removeNotification"
          @mouseenter="setHovered(notification.id, true)"
          @mouseleave="setHovered(notification.id, false)"
        />
      </transition-group>
    </div>
  </template>
  
  <script>
  import GenericNotification from './GenericNotification.vue'
  
  export default {
    components: {
        GenericNotification
    },
    data() {
      return {
        notifications: [],
        nextId: 1,
        hoverStates: {}
      }
    },
    methods: {
      addNotification(notification) {
        const id = this.nextId++
        this.notifications.push({ ...notification, id })
        this.setRemovalTimeout(id)
      },
      removeNotification(id) {
        const index = this.notifications.findIndex(n => n.id === id)
        if (index !== -1) {
          this.notifications.splice(index, 1)
        }
        delete this.hoverStates[id]
      },
      setHovered(id, isHovered) {
        this.hoverStates[id] = isHovered
        if (!isHovered) {
          this.setRemovalTimeout(id)
        }
      },
      setRemovalTimeout(id) {
        setTimeout(() => {
          if (!this.hoverStates[id]) {
            this.removeNotification(id)
          } else {
            this.setRemovalTimeout(id)
          }
        }, 5000)
      }
    }
  }
  </script>
  
  <style scoped>
  .notification-container {
    position: fixed;
    bottom: 20px;
    right: 20px;
    z-index: 1000;
  }
  
  .notification-list-enter-active,
  .notification-list-leave-active {
    transition: all 0.5s ease;
  }
  
  .notification-list-enter-from,
  .notification-list-leave-to {
    opacity: 0;
    transform: translateY(30px);
  }
  
  .notification-list-move {
    transition: transform 0.5s ease;
  }
  </style>