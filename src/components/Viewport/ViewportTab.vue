<script setup>
import FileIcon from '../Icons/FileIcon.vue';

defineProps(['title'])
</script>

<template>
    <div class="row outline-r" style="width: 6rem;">
        <div class="viewport-tab row tab-outline" style="flex-grow: 1; align-items: center; justify-content: center;">
            <FileIcon style="height: 40%; margin-right: 3px; margin-top: -3px;" />
            <a>{{ title }}</a>
        </div>
    </div>
</template>