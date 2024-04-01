export function populateTextEditor(textEditor: HTMLElement, data)
{
    var lineContainer = textEditor;
    lineContainer.innerHTML = "";
    console.log(lineContainer);
    
    for (let i = 0; i < data.content.length; i++)
    {
        var line = document.createElement('div');
        line.id = "line-" + i;
        line.className = "line";

        line.addEventListener("mousedown", function ()
        {
            var selected = lineContainer.querySelector('.line.selected-line');
            if (selected) selected.classList.remove('selected-line');
            line.classList.add('selected-line');
        });

        var index = document.createElement('div');
        index.id = "index";
        index.className = "index";
        index.innerHTML = (i + 1).toString();

        var content = document.createElement('div');
        content.id = "content";
        content.className = "content";
        content.innerHTML = data.content[i];

        line.appendChild(index);
        line.appendChild(content);
        lineContainer.appendChild(line);
    }
}