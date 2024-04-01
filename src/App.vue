<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

</script>

<template>
  <div class="base_container" style="max-width: 100%; max-height: 100%;">
    <div id="title-bar" data-tauri-drag-region class="titlebar row outline-b color">
      <div class="row" style="margin-right: auto; align-items: center;">
        <img src="/tauri.svg" style="padding: 7px; height: 70%;" />
        <button class="context-button">File</button>
        <button class="context-button">Edit</button>
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
    <div class="row" style="flex-grow: 1; left: 0;">
      <div id="function-bar" class="function-bar column color">
        <FileIcon class="function-button selected" />
        <SearchIcon class="function-button" />
        <div style="margin-top: auto;">
          <ThemeIcon class="function-button" />
          <SettingsIcon class="function-button" />
        </div>
      </div>
      <div id="nav-bar" class="nav-bar column color outline-r" style="width: 20em; z-index: 2;">
        <div class="resize-ew disableSelection" style="margin-left: auto;"></div>
      </div>
      <div class="column" style="flex-grow: 1;">
        <div class="row" style="flex-grow: 1;">
          <div id="viewport" class="column" style="flex-grow: 1;">
            <div id="viewport-header" class="viewport-header row color outline-b">
              <ViewportTab title="Wildcard.txt" />
              <ViewportTab title="Tab 2" />
            </div>
            <div id="viewport-content" style="flex-grow: 1;">
              <button @click="loadWildcard">
                click
              </button>
              <TextEditor id="text-editor-0" />
            </div>
          </div>
        </div>
        <div id="context-menu" class="context-menu row color outline-t" style="height: var(--context-menu-height);">
          <div class="resize-ns disableSelection">

          </div>
        </div>
      </div>

    </div>

  </div>
</template>
<script lang="ts">
import { ref } from "vue";
import { appWindow } from '@tauri-apps/api/window'
import { invoke } from "@tauri-apps/api/tauri";
import { populateTextEditor } from './textEditor';

import { setupResize } from './setupResize'
import FileIcon from './components/Icons/FileIcon.vue'
import SearchIcon from './components/Icons/SearchIcon.vue'
import ThemeIcon from './components/Icons/ThemeIcon.vue'
import SettingsIcon from './components/Icons/SettingsIcon.vue'
import TextEditor from './components/Viewport/TextEditor.vue'
import ViewportTab from './components/Viewport/ViewportTab.vue'

export default {

  name: "MainWindow",
  components: {
    FileIcon,
    SearchIcon,
    ThemeIcon,
    SettingsIcon,
    TextEditor,
    ViewportTab
  },
  async mounted()
  {
    await setup();
  },
}

async function setup()
{
  setupResize();
  document.getElementById('titlebar-minimize')?.addEventListener('click', () => appWindow.minimize());
  document.getElementById('titlebar-maximize')?.addEventListener('click', () => appWindow.toggleMaximize());
  document.getElementById('titlebar-close')?.addEventListener('click', () => appWindow.close());
}

async function loadWildcard()
{
  var item = document.getElementById('text-editor-0')!;
  const text = ref();
  text.value = await invoke('load_wildcard');
  console.log(text.value);

  populateTextEditor(item, text.value);
}

</script>