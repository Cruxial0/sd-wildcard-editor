<template>
    <div class="column">
        <div class="row">
            <div class="column">
                <label>{{ name }}</label>
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
        </div>
        <div class="row">
            <MergePatternLine>
                <MergePatternItem></MergePatternItem>
                <MergePatternItem></MergePatternItem>
            </MergePatternLine>
        </div>
    </div>
</template>

<script lang="ts">
import MergePatternItem from './MergePatternItem.vue';
import MergePatternLine from './MergePatternLine.vue';

export default {
    props: ['name'],
    data: (instance) =>
    {
        {name: instance.name}
    }
}
</script>