<script setup lang="ts">
import { Ref, ref } from 'vue'
import{ SortableEvent, VueDraggable, vDraggable} from 'vue-draggable-plus'

const disabled = ref(false);

function onUpdate(event: SortableEvent)
{
    console.log(event);
}
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
function onDragEnd(event: DragEvent)
{
    console.log("drag end");
}
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
                <div v-for="(item, i) in lineCollection" :key="`line_${i}`" v-draggable="item" group="lines" class="merge-editor-line" :disabled="disabled" :animation="150" ghostClass="ghost" @onUpdate="onUpdate">
                    <div class="merge-editor-line-border"/>
                    <MergePatternItem v-for="it in item" :key="it.order" :name="it.name" :kind="it.kind" @click="toggle($event, it)">
                    </MergePatternItem>
                </div>
            </div>
            <div id="add-field" class="add-field" @dragenter="onEnter" @dragleave="onLeave" @dragover="onDragEnd">
                <FileIcon></FileIcon>
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import MergePatternItem from './MergePatternItem.vue'
import MergePatternLine from './MergePatternLine.vue'
import FileIcon from '../Icons/FileIcon.vue';
import { getUUID } from '../../ts/uuid';

const items = ref();
const mergePatterns = ref();
// {name: string, kind: string, id: number, order: number}
const lineCollection = ref();

const inputDev = ref('');

let order = 0;

export default {
    components: {
        MergePatternItem,
        MergePatternLine,
    },
    props: ['name'],
    data() 
    {
        return {
            drag: false
        }
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
            let inputName = inputDev.value == '' ? 'newItem' + '-' + order.toString() : inputDev.value;
            let item = { name: inputName, kind: '0', id: id, order: order };
            lineCollection.value[lineCollection.value.length - 1].push(item);
            items.value.push(item);
            order++;
        },
        setData(data)
        {

            let newData = new Array();
            mergePatterns.value = [];
            lineCollection.value = [];
            data.forEach(pattern => {mergePatterns.value.push(pattern.merge_pattern)});
            mergePatterns.value.forEach(line =>
            {   
                
                console.log(line.merge_pattern);
                line.merge_pattern.forEach(element =>
                {
                    let lineData = new Array();
                    element.forEach(elem =>
                    {
                        console.log(elem);
                        lineData.push({ name: elem.merge_pattern, kind: elem.kind, id: elem.id, order: order });
                        order++;
                    });
                    newData.push(lineData);
                });
                
                lineCollection.value = newData;
                
            });
            items.value = newData;
            
            this.$forceUpdate;
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