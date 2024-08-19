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
    </div>
</template>

<script lang="ts">
import ComboIcon from '../Icons/ComboIcon.vue';
import FileIcon from '../Icons/FileIcon.vue';

export default {
    components: {
        FileIcon,
        ComboIcon
    },
    props: ["name", "kind"]
}
</script>