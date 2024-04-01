<script setup>
defineProps(['title'])
</script>

<template>
    <div class="row outline-r" style="width: 6rem;">
        <div class="viewport-tab row tab-outline" style="flex-grow: 1; align-items: center; justify-content: center;">
            {{ title }}
        </div>
    </div>
</template>