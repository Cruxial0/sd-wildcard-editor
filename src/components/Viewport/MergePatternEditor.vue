<script setup lang="ts">
import { ref } from 'vue'
import{ VueDraggable} from 'vue-draggable-plus'

const disabled = ref(false)
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
                <VueDraggable class="merge-editor-line" v-model="itemsCollection" :disabled="disabled" :animation="150" ghostClass="ghost">
                    <MergePatternItem v-for="item in itemsCollection" :key="item.order" :name="item.name" :kind="item.kind"
                        @click="toggle($event)">
                    </MergePatternItem>
                </VueDraggable>
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import MergePatternItem from './MergePatternItem.vue'
import MergePatternLine from './MergePatternLine.vue'

const inputDev = ref('');
const itemsCollection = ref(new Array());
let newId = 7277;

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
        toggle(event)
        {

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
            let inputName = inputDev.value == '' ? 'newItem' : inputDev.value;
            itemsCollection.value.push({ name: inputName, kind: '0', id: newId });
            newId++;
        },
        setData(data)
        {
            let newData = new Array();
            let order = 0;
            data.forEach(element => {
                newData.push({ name: element.name, kind: element.kind, id: element.id, order: order });
                order++;
            });
            itemsCollection.value = newData;
            newId = order;
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