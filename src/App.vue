<template>
  <div class="base_container" style="position: absolute; top: 0; left: 0; bottom: 0; right: 0; overflow:visible;">
    <div id="title-bar" data-tauri-drag-region class="titlebar row outline-b color"
      style="position: absolute !important; left: 0; top:0; right: 0;">
      <div class="row" style="margin-right: auto; align-items: center; ">
        <img src="/tauri.svg" style="padding: 7px; height: 70%;" />
        <button class="context-button" @click="showPopup = true">File</button>
        <button class="context-button" @click="showNotification">Edit</button>
        <button class="context-button">Selection</button>
        <button class="context-button">View</button>
      </div>

      <div class="titlebar-button" id="titlebar-minimize">
        <img src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize" class="filter-white" />
      </div>
      <div class="titlebar-button" id="titlebar-maximize">
        <img src="https://api.iconify.design/mdi:window-maximize.svg" alt="maximize" class="filter-white" />
      </div>
      <div class="titlebar-button" id="titlebar-close">
        <img src="https://api.iconify.design/mdi:close.svg" alt="close" class="filter-white" />
      </div>

    </div>
    <div id="function-bar-container" class="color" style="position: absolute;left: 0px; bottom: 0px; top: var(--title-bar-height); right: calc(100% - var(--function-bar-width));">
      <div id="function-bar" class="function-bar column">
        <FileIcon class="function-button selected" />
        <SearchIcon class="function-button" />
        <div style="margin-top: auto;">
          <ThemeIcon class="function-button" />
          <SettingsIcon class="function-button" />
        </div>
      </div>
    </div>
    <div id="nav-bar-container" style="position: absolute; left: var(--function-bar-width); right: calc(100vw - var(--function-bar-width) - var(--nav-bar-width)); bottom: 0; top: var(--title-bar-height);">
      <div id="nav-bar" class="nav-bar row color outline-r">
        <div class="resize-ew disableSelection" style="margin-left: auto;"></div>
        <div id="nav-bar-content" class="nav-bar-content outline-b">
          <Suspense>
            <ProjectExplorer style="margin-top: 5px;" />
          </Suspense>
        </div>
      </div>
    </div>
    <div id="viewport-container" style="position: absolute; left: calc(var(--function-bar-width) + var(--nav-bar-width)); right: 0; top: var(--title-bar-height); bottom: calc(var(--context-menu-height));">
      <div class="row" style="flex-grow: 1;">
        <div id="viewport" class="column" style="flex-grow: 1;">
          <div id="viewport-scroll-container">
            <div id="viewport-header" class="viewport-header row color outline-b">
              <ViewportTab viewportTitle="Wildcard.txt" />
              <ViewportTab viewportTitle="Tab 2" />
            </div>
          </div>
          <div id="viewport-content" class="viewport-container">

          </div>
        </div>
      </div>
    </div>
    <div id="context-menu-container" class="color outline-t" style="position: absolute; left: calc(var(--function-bar-width) + var(--nav-bar-width)); right: 0; bottom: 0; top: calc(100vh - var(--context-menu-height));">
      <div id="context-menu" class="context-menu row ">
        <div class="resize-ns disableSelection">

        </div>
      </div>
    </div>

  </div>
  <FileEntryCM></FileEntryCM>
  <ComboWildcardCM></ComboWildcardCM>
  <GenericPopup :isVisible="showPopup" @close="showPopup = false">
    <h2>{{ popupTitle }}</h2>
    <p>{{ popupContent }}</p>
  </GenericPopup>
  <NotificationManager ref="notificationManager" />
</template>
<script lang="ts">
import { appWindow } from '@tauri-apps/api/window'


import { setupResize } from './setupResize'
import { initializeDefaultSelectionListeners } from './selectionListeners'
import FileIcon from './components/Icons/FileIcon.vue'
import SearchIcon from './components/Icons/SearchIcon.vue'
import ThemeIcon from './components/Icons/ThemeIcon.vue'
import SettingsIcon from './components/Icons/SettingsIcon.vue'
import TextEditor from './components/Viewport/TextEditor.vue'
import ViewportTab from './components/Viewport/ViewportTab.vue'
import ProjectExplorer from './components/NavBar/ProjectExplorer.vue'
import GenericPopup from './components/Popup/GenericPopup.vue'
import NotificationManager from './components/Notification/NotificationManager.vue'
import GenericNotification from './components/Notification/GenericNotification.vue'
import FileEntryCM from './components/ContextMenu/FileEntryCM.vue'
import ComboWildcardCM from './components/ContextMenu/ComboWildcardCM.vue'
import MergePatternEditor from './components/Viewport/MergePatternEditor.vue'
import {VueDraggable} from 'vue-draggable-plus'

export default {

  name: "MainWindow",
  components: {
    FileIcon,
    SearchIcon,
    ThemeIcon,
    SettingsIcon,
    TextEditor,
    ViewportTab,
    ProjectExplorer,
    GenericPopup,
    NotificationManager,
    GenericNotification,
    FileEntryCM,
    ComboWildcardCM,
    MergePatternEditor,
    VueDraggable
  },
  async mounted()
  {
    await setup();
  },
  data()
  {
    return {
      showPopup: false,
      popupTitle: 'Dynamic Title',
      popupContent: 'This is dynamic content, which is substantially longer to test if the window dynamically resizes or not. Bruh why are you even reading this lol'
    }
  },
  methods:
  {
    showNotification()
    {
      console.log("showing notification");
      this.$refs.notificationManager.addNotification({
        icon: 'CheckCircleIcon',
        header: 'Success!',
        message: 'Your action was completed successfully.',
        borderColor: '#2ecc71'
      })
    }
  }
}

async function setup()
{
  window.addEventListener('DOMContentLoaded', () => setupResize());
  
  initializeDefaultSelectionListeners();
  document.getElementById('titlebar-minimize')?.addEventListener('click', () => appWindow.minimize());
  document.getElementById('titlebar-maximize')?.addEventListener('click', () => appWindow.toggleMaximize());
  document.getElementById('titlebar-close')?.addEventListener('click', () => appWindow.close());
}

</script>