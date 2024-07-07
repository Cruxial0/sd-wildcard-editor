<template>
    <div>
      <transition name="overlay">
        <div v-if="isVisible" class="popup-overlay" @click="close"></div>
      </transition>
      <transition name="popup">
        <div v-if="isVisible" class="popup-content">
          <slot></slot>
          <button @click="close">Close</button>
        </div>
      </transition>
    </div>
  </template>
  
  <script>
  export default {
    props: {
      isVisible: Boolean
    },
    methods: {
      close() {
        this.$emit('close')
      }
    }
  }
  </script>
  
  <style scoped>
  .popup-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    z-index: 198;
  }
  
  .popup-content {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    background-color: var(--background-color);
    padding: 20px;
    border-radius: 5px;
    z-index: 199;
  }
  
  /* Overlay animation */
  .overlay-enter-active,
  .overlay-leave-active {
    transition: opacity 0.1s ease;
  }
  
  .overlay-enter-from,
  .overlay-leave-to {
    opacity: 0;
  }
  
  /* Popup content animation */
  .popup-enter-active,
  .popup-leave-active {
    transition: all 0.1s ease;
  }
  
  .popup-enter-from,
  .popup-leave-to {
    opacity: 0;
    transform: translate(-50%, -50%) scale(0.8);
  }
  
  .popup-enter-to,
  .popup-leave-from {
    opacity: 1;
    transform: translate(-50%, -50%) scale(1);
  }
  </style>
  