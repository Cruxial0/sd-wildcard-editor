<script setup lang="ts">
import { ref } from 'vue';
import { AddViewportMergePattern, AddViewportTab, DisplayViewport } from '../../ts/viewport/viewportHelper';
import { invoke } from '@tauri-apps/api';
import ComboIcon from '../Icons/ComboIcon.vue';
import CloseIcon from '../Icons/CloseIcon.vue';

async function createViewport()
{
    let comboWildcard = ref();
    let callerId = document.getElementById('cm-combo-wildcard-entry')?.getAttribute('callerId'); 
    comboWildcard.value = await invoke('load_merge_definition_from_subject', { id: callerId });

    let id = AddViewportMergePattern(callerId, comboWildcard.value); 
    await AddViewportTab(id);
    await DisplayViewport(id);
}
</script>

<template>
    <div id="cm-combo-wildcard-entry" class="rmb-context-menu">
        <ul>
            <li><i class="rmb-context-menu-icon"></i>Edit</li>
            <li><i class="rmb-context-menu-icon"><CloseIcon/></i>Delete</li>
            <li @click="createViewport"><i class="rmb-context-menu-icon"><ComboIcon/></i>Merge Patterns</li>
        </ul>
    </div>
</template>

<script lang="ts">
export default {
    components: {
        CloseIcon,
        ComboIcon
    }
}
</script>