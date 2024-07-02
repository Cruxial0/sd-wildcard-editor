import { Component, ComponentPublicInstance, ComputedOptions, MethodOptions, createApp } from "vue";
import TextEditor from "../../components/Viewport/TextEditor.vue";
import { ViewportItem } from "./viewportItem";
import { ViewportTextEditor } from "./viewportTextEditor";
import ViewportTab from "../../components/Viewport/ViewportTab.vue";

const viewportElementId = 'viewport-content';
const viewportTabElementId = 'viewport-header';

let viewports: Map<number, ViewportItem> = new Map();
let viewportTabs: Map<number, ComponentPublicInstance> = new Map();
let viewportWildcards: Map<number, number> = new Map();

var loadedViewport: number = -1;
var maxId: number = 0;

export function AddViewportTextEditor(id: number): number
{
    console.log('INITIAL ID: ' + id);
    if (viewportWildcards.has(id)) return viewportWildcards.get(id)!;
    
    var textEditor = createInstance(TextEditor, {})
    var item = new ViewportTextEditor(textEditor.$el, id);
    viewports.set(maxId, item);
    viewportWildcards.set(id, maxId);
    maxId++;
    return maxId - 1;
}

export function  DisplayViewport(id: number): void;
export function  DisplayViewport(id: number, element: HTMLElement): void;
export function  DisplayViewport(id: number, element?: HTMLElement): void
{
    console.log(viewportTabs);
    let elem = element ? element : document.getElementById(viewportElementId)!;
    let viewport = viewports.get(id);
    if (viewport)
    {
        if (loadedViewport != -1) UnloadViewport(loadedViewport);
        loadedViewport = id;
        viewport.display(elem);

        let tab = viewportTabs.get(id);
        if (tab)
        {
            var selected = document.querySelector('.' + 'viewport-tab' + '.' + 'selected-tab');
            if (selected) selected.classList.remove('selected-tab');
            tab.$el.querySelector('.viewport-tab').classList.add('selected-tab');
        }
    }
}

export function  UnloadViewport(id: number)
{
    let viewport = viewports.get(id);
    if (viewport) viewport.unload();
    document.getElementById(viewportElementId)!.innerHTML = '';
}

export function RemoveViewport(id: number)
{
    let viewport = viewports.get(id);
    let dataId = viewport?.getDataId();

    viewportWildcards.delete(dataId!);

    if (viewport)
    {
        viewport.unload();
        let tab: HTMLElement = viewportTabs.get(id)!.$el;
        tab.outerHTML = '';
    }

    viewports.delete(id);
    viewportTabs.delete(id);
    document.getElementById(viewportElementId)!.innerHTML = '';
}

export async function AddViewportTab(id: number)
{
    var elem = document.getElementById(viewportTabElementId)!;
    let tab = await CreateViewportTab(id);
    console.log(tab);
    
    if (tab)
    {
        console.log(viewportTabs.has(id));
        console.log('ID:' + id);
        if (viewportTabs.has(id)) return;
        console.log("why are u here");

        elem.appendChild(tab.$el);
        viewportTabs.set(id, tab);
        console.log(viewportTabs.has(id));

        var selected = document.querySelector('.' + 'viewport-tab' + '.' + 'selected-tab');
        if (selected) selected.classList.remove('selected-tab');
        tab.$el.querySelector('.viewport-tab').classList.add('selected-tab');
    }
}

export async function  CreateViewportTab(id: number): Promise<ComponentPublicInstance | undefined>
{
    let item = viewports.get(id);

    if (item)
    {
        console.log(await item.getName());
        let tab = createInstance(ViewportTab, { viewportTitle: await item.getName(), itemId: id });
        return tab;
    }

    return undefined;
}

// export function InitializeViewportHelper()
// {
//     if (!viewports) viewports = new Map();
//     if (!viewportTabs) viewportTabs = new Map();
// }

function createInstance(rootComponent: Component<any, any, any, ComputedOptions, MethodOptions, {}, any>, componentProperties): ComponentPublicInstance
{
    const tempDiv = document.createElement('div');
    return createApp(rootComponent, componentProperties).mount(tempDiv);
}