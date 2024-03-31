import 'vue-resizable'

const contextMenuSizeHeightVar = '--context-menu-size';
const BORDER_SIZE = 4;
let m_pos;

let contextMenuResize: HTMLElement;
let contextMenu;

function resize_x(element, e){
  const dx = m_pos - e.x;
  m_pos = e.x;
  element.style.width = (parseInt(getComputedStyle(element, '').width) + dx) + "px";
}

function resize_y(e){
    const dy = m_pos - e.y;
    m_pos = e.y;
    
    console.log(contextMenu);
    contextMenu.style.height = (parseInt(getComputedStyle(contextMenu, '').height) + dy) + "px";
}

export function setupResize()
{
    contextMenuResize = document.getElementsByClassName('resize-ns')[0]! as HTMLElement;
    contextMenu = document.getElementById('context-menu');
    contextMenu.addEventListener('mousedown', function (e)
    {
        document.addEventListener("mousemove", resize_y, false);
    });

    document.addEventListener('mouseup', function()
    {
        console.log("remove");
        document.removeEventListener('mousemove', resize_y, false);
    });
}

function update_variable(variable: string, value: string)
{
    document.documentElement.style.setProperty(variable, value);
    console.log(document.documentElement.style.getPropertyValue(variable));
    
}