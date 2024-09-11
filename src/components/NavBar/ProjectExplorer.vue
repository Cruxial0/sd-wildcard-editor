<template>
    <div>
        <a style="margin-top: 10px; margin-left: 15px; font-size: 10px;">EXPLORER</a>
        <div class="outline-b" style="width: 95%; margin: 0px 0px 5px 10px; height: 5px;"></div>
        <a style="margin-left: 15px; font-size: 10px; font-weight:900;">PROJECT</a>
        <div id="file-hierarchy" style="overflow-x: auto; height: calc(100vh - var(--title-bar-height) - var(--title-bar-height) - 38px);">

        </div>
    </div>
</template>

<script lang="ts">
import { buildProjectExplorer } from '../../projectExplorer'
export default {
    async mounted()
    {
        await buildProjectExplorer();
    }
}
</script>