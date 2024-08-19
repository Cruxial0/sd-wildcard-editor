<script lang="ts">
export default {
    props: ['name', 'wildcardId'],
    data: (instance) =>
    {
        return { file: instance.name, id: parseInt(instance.wildcardId) }
    }
}
</script>

<template>
    <div>
        <div id="file-entry" class="file-entry row disableSelection">
            <!--<FileIcon class="file-icon" style="height: 100%; margin-bottom: 5px;" />-->
            <a style="margin-left: 5px;">{{ name }}</a>
        </div>
        <div id="children" class="column" style="padding-left: 15px;">

        </div>
    </div>
</template>