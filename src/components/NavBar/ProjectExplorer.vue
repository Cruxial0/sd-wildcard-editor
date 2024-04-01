<script setup lang="ts">
import FileIcon from '../Icons/FileIcon.vue';
import FileIndicator from './FileIndicator.vue'
import { populateTextEditor } from '../../textEditor.ts';
import { ref, createApp } from "vue";
import { invoke } from "@tauri-apps/api/tauri";



async function click()
{
    
    const text = ref();

    text.value = await invoke('load_wildcard', { name: "wildcard" });
    console.log(text.value);
    
}

</script>

<template>
    <div>
        <a style="margin-top: 10px; margin-left: 15px; font-size: 10px;">EXPLORER</a>
        <div class="outline-b" style="width: 95%; margin: 0px 0px 5px 10px; height: 5px;"></div>
        <a style="margin-left: 15px; font-size: 10px; font-weight:900;">PROJECT</a>
        <div id="file-hierarchy">

        </div>
    </div>
</template>

<script lang="ts">
export default {
    async mounted()
    {
        await setup();
    }
}

async function setup()
{
    const files = ref();
    files.value = await invoke('load_wildcards');
    var hierarchy = document.getElementById('file-hierarchy')!;
    var item = document.getElementById('text-editor-0')!;

    for (let i = 0; i < files.value.length; i++)
    {
        const tempDiv = document.createElement('div');
        const instance = createApp(FileIndicator, { name: files.value[i].name }).mount(tempDiv);
        instance.$el.addEventListener("mousedown", async function ()
        {
            var wildcardName = instance.$data.file.replace(/\.[^/.]+$/, "");
            const wildcard = ref();
            wildcard.value = await invoke("load_wildcard", { name: wildcardName });
            
            populateTextEditor(item, wildcard.value);


        });
        hierarchy.appendChild(instance.$el);
    }
}
</script>