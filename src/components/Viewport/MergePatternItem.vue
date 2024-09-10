<template>
    <div id="merge-item-container" class="merge-item-container">
        <div id="merge-item-icon" style="display:flex; flex-shrink: 0; width: 20px; height: 100%; background-color: var(--context-menu-color); pointer-events: none; padding-left: 5px;">
            <div v-if="kind===0" style="display: flex; align-self: center; justify-content: center;">
                <FileIcon/>
            </div>
            <div v-else style="display: flex; align-self: center; justify-content: center;">
                <ComboIcon/>
            </div>
        </div>
        <div id="merge-item-content" class="merge-item">
            {{ name }}
        </div>
        <div id="open-tab" style="display: none; align-self: center; justify-content: center; width: 15px; height: 100%; padding-left: 5px; padding-right: 5px; background-color: var(--context-menu-color);">
            <OpenTabIcon @click="openTab"/>
        </div>
    </div>
</template>

<script lang="ts">
import { ref } from 'vue';
import ComboIcon from '../Icons/ComboIcon.vue';
import FileIcon from '../Icons/FileIcon.vue';
import OpenTabIcon from '../Icons/OpenTabIcon.vue';
import { AddViewportMergePattern, AddViewportTab, AddViewportTextEditor, DisplayViewport } from '../../ts/viewport/viewportHelper';
import { invoke } from '@tauri-apps/api';

export default {
    components: {
        FileIcon,
        ComboIcon, 
        OpenTabIcon
    },
    props: ["id", "name", "kind"],
    methods: {
        async openTab()
        {
            let item = ref();
            let dataId = this.$props.id;
            let kind = this.$props.kind;

            if (kind == 0)
            {
                let id = AddViewportTextEditor(dataId); 
                await AddViewportTab(id);
                await DisplayViewport(id);
            }

            if (kind == 1)
            {
                item.value = await invoke('load_merge_definition_from_subject', { id: dataId });

                let id = AddViewportMergePattern(dataId, item.value); 
                await AddViewportTab(id);
                await DisplayViewport(id);
            }
            
        }
    }
}
</script>