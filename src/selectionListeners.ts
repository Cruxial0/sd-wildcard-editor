export function addSelectionListener(element, baseClass, selectedClass)
{
    element.addEventListener("mousedown", function ()
    {
        var selected = document.querySelector('.' + baseClass + '.' + selectedClass);
        if (selected) selected.classList.remove(selectedClass);
        element.classList.add(selectedClass);
    });
}

export function addSelectionListeners(elements, baseClass, selectedClass)
{
    for (let i = 0; i < elements.length; i++)
        addSelectionListener(elements[i], baseClass, selectedClass);
}

export function initializeDefaultSelectionListeners()
{
    var funcButtons = document.getElementsByClassName('function-button');
    addSelectionListeners(funcButtons, 'function-button', 'selected');

    var tabs = document.getElementsByClassName('viewport-tab');
    addSelectionListeners(tabs, 'viewport-tab', 'selected-tab');
}