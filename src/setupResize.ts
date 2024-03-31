import 'vue-resizable'

let m_pos_x;
let m_pos_y;
let active: HTMLElement;

function resize_x(e){
    const dx = e.x - m_pos_x;
    m_pos_x = e.x;

    active.style.width = (parseInt(getComputedStyle(active, '').width) + dx) + "px";
}

function resize_y(e){
    const dy = m_pos_y - e.y;
    m_pos_y = e.y;

    active.style.height = (parseInt(getComputedStyle(active, '').height) + dy) + "px";
}

function setupElement(element: HTMLElement, direction: string)
{
    console.log("setting up element: " + element.className);
    
    if (direction != 'x' && direction != 'y')
        console.error("Invalid resize direction: " + direction);

    element.addEventListener('mousedown', function (e)
    {
        active = element.parentNode as HTMLElement;
        console.log(active.id);
        
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
        console.log("remove");
        document.removeEventListener('mousemove', resize_x, false);
        document.removeEventListener('mousemove', resize_y, false);
    });
}

function update_variable(variable: string, value: string)
{
    document.documentElement.style.setProperty(variable, value);
    console.log(document.documentElement.style.getPropertyValue(variable));
    
}