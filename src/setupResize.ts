const cm_variable = "--context-menu-height";
const nb_variable = "--nav-bar-width";
let m_pos_x: number;
let m_pos_y: number;
let resizeable: Map<string, number> = new Map();
let active = '';

function update_variable(variable: string, value: string)
{
    document.documentElement.style.setProperty(variable, value);
}

function resize_x(e){
    const dx = e.x - m_pos_x;
    m_pos_x = e.x;
    
    let min = resizeable.get(active)!;
    let item = document.getElementById(active)!;
    let currWidth = parseInt(getComputedStyle(item, '').width);
    
    var val = currWidth + dx < min ? "180px" : (currWidth + dx) + "px";
    update_variable(nb_variable, val);
}

function resize_y( e){
    const dy = m_pos_y - e.y;
    m_pos_y = e.y;

    let min = resizeable.get(active)!;
    let item = document.getElementById(active)!;
    let currHeight = parseInt(getComputedStyle(item, '').height);

    var val = currHeight + dy < min ? "180px" : (currHeight + dy) + "px";
    update_variable(cm_variable, val);
}

function setupElement(element: HTMLElement, direction: string)
{
    console.log("setting up element: " + element.className);
    let parent = element.parentNode?.parentElement as HTMLElement;
    if (resizeable.has(parent.id)) return;

    if (direction == 'x') resizeable.set(parent.id, parseInt(getComputedStyle(parent).width, 10));
    if (direction == 'y') resizeable.set(parent.id, parseInt(getComputedStyle(parent).height, 10));
    
    if (direction != 'x' && direction != 'y')
    {
        console.error("Invalid resize direction: " + direction);
        return;
    }
        
    element.addEventListener('mousedown', function (e)
    {
        active = parent.id;

        m_pos_x = e.x;
        m_pos_y = e.y;
        
        document.addEventListener("mousemove", direction == 'x' ? resize_x : resize_y, false);
    });
}

function setupList(elems: HTMLCollectionOf<Element>, direction: string)
{
    for (let i = 0; i < elems.length; i++)
    {
        setupElement(elems[i] as HTMLElement, direction);
    }
}

export function setupResize()
{
    var nsElems = document.getElementsByClassName('resize-ns');
    var ewElems = document.getElementsByClassName('resize-ew');

    setupList(nsElems, 'y');
    setupList(ewElems, 'x');

    document.addEventListener('mouseup', function()
    {
        document.removeEventListener('mousemove', resize_x, false);
        document.removeEventListener('mousemove', resize_y, false);
    });
}