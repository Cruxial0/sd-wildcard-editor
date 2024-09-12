<template>
    <suspense>
        <div id="editor" class="editor">
            <div v-for="(line, index) in documentModel.lines" :key="index" class="line">
                <template v-for="node in line" :key="node.id">
                    <component :is="getComponent(node)" :node="node"></component>
                </template>
            </div>
        </div>
    </suspense>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { DocumentNode } from '../../ts/textEditor/nodes/baseNode';
import TextNodeComponent from './Components/TextNodeComponent.vue';
import ComponentNodeResolver from './Components/ComponentNodeResolver.vue';
import { DocumentModel } from '../../ts/textEditor/document';
import DelimiterNodeComponent from './Components/DelimiterNodeComponent.vue';

export default defineComponent({
    props: ['uuid'],
    setup(props)
    {
        const text = "this is an example string, containing  __wildcards__. multiple __wildcard__ instances. å こし"
        const documentModel = ref(new DocumentModel(props.uuid));
        documentModel.value.load();

        const getComponent = (node: DocumentNode) => 
        {
            switch (node.type)
            {
                case 'text': return TextNodeComponent;
                case 'component': return ComponentNodeResolver;
                case 'delimiter': return DelimiterNodeComponent;
            }
        };

        const handleInput = (event: InputEvent) => 
        {
            // Handle input and update the document model
        };

        return { documentModel, getComponent, handleInput };
    },
});
</script>