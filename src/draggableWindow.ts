// You can choose to have an element with the class "window-top" inside of your draggable window that will act as the "handle" for the window or it will attach to the element itself

let dragComponent = document.getElementById('action-bar');
let dragElement = '.action-bar';

    export function initializeDragElement()
    {
        if(dragComponent !== null) makeDraggable(dragComponent);
    }

function makeDraggable(elmnt: HTMLElement) 
{
    // Make an element draggable (or if it has a .window-top class, drag based on the .window-top element)
    let currentPosX = 0, currentPosY = 0, previousPosX = 0, previousPosY = 0;

		// If there is a window-top classed element, attach to that element instead of full window
    if (elmnt.querySelector(dragElement) !== null) {
        // If present, the window-top element is where you move the parent element from
        elmnt.querySelector(dragElement)?.addEventListener('mousedown', onmousedown);
    } 
    else {
        // Otherwise, move the element itself
        elmnt.onmousedown = onmousedown;
    }

    function onmousedown(ev: Event) {
        // Prevent any default action on this element (you can remove if you need this element to perform its default action)
        ev.preventDefault();
        // Get the mouse cursor position and set the initial previous positions to begin
        previousPosX = (ev as MouseEvent).clientX;
        previousPosY = (ev as MouseEvent).clientY;
        // When the mouse is let go, call the closing event
        document.onmouseup = closeDragElement;
        // call a function whenever the cursor moves
        document.onmousemove = elementDrag;
    }

    function elementDrag (e: MouseEvent) {
        // Prevent any default action on this element (you can remove if you need this element to perform its default action)
        e.preventDefault(); 
        // Calculate the new cursor position by using the previous x and y positions of the mouse
        currentPosX = previousPosX - e.clientX;
        currentPosY = previousPosY - e.clientY;
        // Replace the previous positions with the new x and y positions of the mouse
        previousPosX = e.clientX;
        previousPosY = e.clientY;
        // Set the element's new position
        elmnt.style.top = (elmnt.offsetTop - currentPosY) + 'px';
        elmnt.style.left = (elmnt.offsetLeft - currentPosX) + 'px';
    }

    function closeDragElement () {
        // Stop moving when mouse button is released and release events
        document.onmouseup = null;
        document.onmousemove = null;
    }
}