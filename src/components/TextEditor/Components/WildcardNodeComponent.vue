<template>
    <span class="gtk1 wildcard" contenteditable="true">{{ node.props.content }}</span>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    props: ['node']
});
</script>