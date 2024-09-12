<template>
    <a class="selectedSpan" contenteditable="true">{{ node.content }}</a>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    props: ['node']
})
</script>