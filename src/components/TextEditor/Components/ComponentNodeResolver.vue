<template>
    <component :is="getComponent(node)" :node="node"></component>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { ComponentNode } from '../../../ts/textEditor/nodes/baseNode';
import WildcardNodeComponent from './WildcardNodeComponent.vue';
import NumberNodeComponent from './NumberNodeComponent.vue';

export default defineComponent({
    props: ['node'],
    setup()
    {
        const getComponent = (node: ComponentNode) => 
        {
            switch (node.componentType)
            {
                case 'wildcard': return WildcardNodeComponent;
                case 'number': return NumberNodeComponent;
                default: break;
            }
        };

        return { getComponent }
    }
});
</script>