<script setup lang="ts">
import { ref } from 'vue'
import { VueDraggable } from 'vue-draggable-plus'
import MergePatternItem from './MergePatternItem.vue'
import MergePatternLine from './MergePatternLine.vue'
import FileIcon from '../Icons/FileIcon.vue';


// function onUpdate(event: SortableEvent)
// {
//     console.log(lineCollection);
// }
function onEnter(event: DragEvent)
{
    console.log(event);
    let t = event.target as HTMLElement;
    console.log(t);
    t.classList.add('add-hover');
}
function onLeave(event: DragEvent)
{
    let t = event.target as HTMLElement;
    t.classList.remove('add-hover');
}
// function onDragEnd(event: DragEvent)
// {
//     console.log("drag end");
// }
</script>

<template>
    <div class="column merge-editor-container">
        <div class="row">
            <div class="column">
                <label class="merge-title">{{ name }}</label>
                <select>
                    <option>Individual</option>
                    <option>Combined</option>
                    <optgroup>
                        <option>Saved Custom 1</option>
                        <option>Saved Custom 2</option>
                    </optgroup>
                    <option>Custom</option>
                </select>
            </div>
            <div class="column" style="margin-left: auto;"></div>
            <div class="column" style="margin-left: 10px;">
                <div class="row">
                    <input id="merge-add-dev" v-model="inputDev">
                    <button @click="addItem" style="padding: 5px; border-radius: 5px; margin-left: 5px;">Add</button>
                </div>
            </div>
        </div>
        <div id="merge-editor-lines">
            <div style="margin-top: 10px;">
                <VueDraggable v-model="lineCollection" group="lines" :animation="150" ghostClass="ghost" target=".merge-editor-line" handle=".merge-item-container">
                    <div id="lines" class="merge-editor-line" v-for="line in lineCollection">
                        <div class="merge-editor-line-border"/>
                        <MergePatternItem v-for="it in line" :key="it.id" :id="it.id" :name="it.name" :kind="it.kind" @click="toggle($event, it)"/>
                    </div>
                </VueDraggable>
            </div>
            <div id="add-field" class="add-field" @dragenter="onEnter" @dragleave="onLeave" >
                <FileIcon></FileIcon>
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { getNameByUUID, getUUID } from '../../ts/uuid';

const items = ref();
const mergePatterns = ref();
// {name: string, kind: string, id: number, order: number}
const lineCollection = ref();

const inputDev = ref('');

export default {
    components: {
        MergePatternItem,
        MergePatternLine,
    },
    props: ['name', 'mergeDefinitions'],
    async beforeMount()
    {
        let newData = new Array();
        mergePatterns.value = [];
        lineCollection.value = [];
        this.$props.mergeDefinitions.forEach(pattern => { mergePatterns.value.push(pattern.merge_definition) });
        console.log(this.$props.mergeDefinitions);
        mergePatterns.value.forEach(line =>
        {   
            console.log(line);
            console.log(line.merge_pattern);
            line.merge_pattern.forEach(async element =>
            {
                let lineData = new Array();
                element.forEach(async elem =>
                {
                    console.log(elem);
                    lineData.push({ name: elem.node, kind: elem.kind, id: elem.node.replaceAll('__', '') });
                });
                newData.push(lineData);
            });
            
            lineCollection.value = newData;
            
        });
        items.value = newData;
    },
    async mounted()
    {
        lineCollection.value.forEach(async (line) =>
        {
            line.forEach(async (item) => {item.name = await getNameByUUID(item.name.replaceAll('__', ''))})
        });
    },
    methods: {
        toggle(event, debug)
        {
            console.log(debug);
            let element = event.target.id == "merge-item-container" ? event.target : event.target.parentElement;
            if (element.classList.contains('deselect'))
            {
                element.classList.remove('deselect');
                element.style.opacity = 1;
            }
            else
            {
                element.classList.add('deselect');
                element.style.opacity = 0.3;
            }
        },
        addItem()
        {
            let id = getUUID();
            let inputName = inputDev.value == '' ? 'newItem' : inputDev.value;
            let item = { name: inputName, kind: '0', id: id};
            lineCollection.value[lineCollection.value.length - 1].push(item);
            items.value.push(item);
        },
        
    }
}

</script>
<style scoped>
.ghost {
    opacity: 0.5;
    background: #c8ebfb;
}
</style>