<script setup lang="ts">
import FileIcon from '../Icons/FileIcon.vue';
</script>

<script lang="ts">
export default {
    props: ['name'],
    data: (instance) =>
    {
        return {file: instance.name }
    }
}
</script>

<template>
    <div id="file-entry" class="file-entry row">
        <FileIcon class="file-icon" style="height: 100%; margin-bottom: 5px;" />
        <a style="margin-left: 5px; font-size: 10px;">{{ name }}</a>
    </div>
</template>