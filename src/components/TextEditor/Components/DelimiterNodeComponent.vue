<template>
    <div style="width: 1em; height: 100%;">{{ node.delimiter }}</div>    
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    props: ['node']
})
</script>