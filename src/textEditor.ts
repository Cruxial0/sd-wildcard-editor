import { addSelectionListener } from "./selectionListeners";

const SPLIT_REGEX = /(,)/g;

function generateLine(text: string)
{
    var items = text.split(SPLIT_REGEX);
    console.log(items);
    
    let spans: HTMLElement[] = [];

    for (let i = 0; i < items.length; i++)
    {
        var span = document.createElement('span');
        span.innerHTML = items[i];
        spans[i] = span;
    }

    return spans
}

export function populateTextEditor(textEditor: HTMLElement, data)
{
    var lineContainer = textEditor;
    lineContainer.innerHTML = "";
    
    for (let i = 0; i < data.content.length; i++)
    {
        var line = document.createElement('div');
        line.id = "line-" + i;
        line.className = "line";

        addSelectionListener(line, 'line', 'selected-line');

        var index = document.createElement('div');
        index.id = "index";
        index.className = "index";
        index.innerHTML = (i + 1).toString();

        var content = document.createElement('div');
        content.id = "content";
        content.className = "content";
        
        generateLine(data.content[i]).forEach((x) => content.appendChild(x));

        line.appendChild(index);
        line.appendChild(content);
        lineContainer.appendChild(line);
    }
}