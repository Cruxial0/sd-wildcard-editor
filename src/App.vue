<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { appWindow } from '@tauri-apps/api/window'

console.log(document);
</script>

<template>
  <div class="base_container">
    <div id="title-bar" data-tauri-drag-region class="titlebar row outline-b color" style="height: 2em; z-index: 3;">
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
      <div id="function-bar" class="column color" style="width: 4em;">

      </div>
      <div id="nav-bar" class="column color outline-r" style="width: 20em; z-index: 2;">
        <div class="resize-ew disableSelection" style="margin-left: auto;"></div>
      </div>
      <div class="column" style="flex-grow: 1;">
        <div class="row" style="flex-grow: 1;">
          <div id="viewport" class="column" style="flex-grow: 1;">
            <div id="viewport-header" class="row color outline-b" style="height: 2.4em;">
              <div class="row outline-r" style="width: 6rem;">
                <div class="row tab-outline selected-tab " style="flex-grow: 1;">

                </div>
              </div>
              <div class="row outline-r" style="width: 6rem;">
                <div class="row tab-outline" style="flex-grow: 1;">

                </div>
              </div>
            </div>
          </div>
        </div>
        <div id="context-menu" class="row color outline-t" style="height: var(--context-menu-height);">
          <div class="resize-ns disableSelection">

          </div>
        </div>
      </div>

    </div>

  </div>
</template>
<script lang="ts">
import { setupResize } from './setupResize'

export default {

  name: "MainWindow",
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
</script>