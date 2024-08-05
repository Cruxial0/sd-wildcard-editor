<script setup lang="ts">
import { AddViewportMergePattern, DisplayViewport } from '../../ts/viewport/viewportHelper';

async function createViewport()
{
    let id = await AddViewportMergePattern();
    await DisplayViewport(id, document.getElementById('viewport-content')!);
}
</script>

<template>
    <div id="cm-combo-wildcard-entry" class="rmb-context-menu">
        <ul>
            <li><i class="rmb-context-menu-icon icon-edit"></i>Edit</li>
            <li><i class="rmb-context-menu-icon icon-delete"></i>Delete</li>
            <li><i class="rmb-context-menu-icon icon-delete" v-on:click="createViewport"></i>Options</li>
        </ul>
    </div>
</template>