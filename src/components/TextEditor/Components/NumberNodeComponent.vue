<template>
    <a style="color: var(--gtk2);" contenteditable="true">{{ node.props.content }}</a>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    props: ['node'],
    setup(props)
    {
        console.log(props.node)
    }
})
</script>