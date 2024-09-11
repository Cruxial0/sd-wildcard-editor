<script lang="ts">
import { addSelectionListener } from '../../selectionListeners';
import CloseIcon from '../Icons/CloseIcon.vue';
import FileIcon from '../Icons/FileIcon.vue';
import { DisplayViewport, RemoveViewport } from '../../ts/viewport/viewportHelper';

export default {
    props: ['viewportTitle', 'itemId'],
    data: (instance) =>
    {
        return {title: instance.viewportTitle, id: instance.itemId}
    },
    mounted()
    {
        // var helper = new ViewportHelper();
        var id = this.itemId;
        var elem: HTMLElement = this.$el.querySelector('.viewport-tab');
        elem!.addEventListener("click", async function ()
        {
            console.log("base click");
            DisplayViewport(id);
        });
        elem!.querySelector('#tab-close-btn')!.addEventListener("click", async function (e)
        {
            console.log("close click");
            e.stopPropagation();
            RemoveViewport(id);
        });
        addSelectionListener(elem, 'viewport-tab', 'selected-tab');
    },
    components: {
        FileIcon,
        CloseIcon
    }
}
</script>

<template>
    <div class="row outline-r" style="width: auto;">
        <div class="viewport-tab row tab-outline" style="flex-shrink: 1; align-items: center; justify-content: center; padding: 0px 3px 0px 18px;">
            <FileIcon class="file-icon" style="height: 15px; aspect-ratio: 1 / 1; margin-right: 3px; margin-top: -3px;"/>
            <span>{{ viewportTitle }}</span>
            <CloseIcon id="tab-close-btn" class="close-tab" style="height: 15px; aspect-ratio: 1 / 1; margin-left: 5px; margin-top: -1px;"/>
        </div>
    </div>
</template>