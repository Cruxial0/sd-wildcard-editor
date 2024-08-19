let trackedMenus: string[] = [];

export function AddContextMenuHandlers(triggers: string, dataMenus: string, triggerId)
{
    document.addEventListener('DOMContentLoaded', function ()
    {
      const triggerObject = document.querySelectorAll(triggers);
      const menus = document.querySelectorAll<HTMLElement>(dataMenus)!;
    
      triggerObject.forEach(trigger =>
      {
        trigger.addEventListener('contextmenu', function (e)
        {
          e.preventDefault();
          const menuId = trigger.getAttribute('data-menu')!;
          const menu = document.getElementById(menuId)! as HTMLElement;
    
          // Hide all menus
          menus.forEach(m => m.style.display = 'none');
    
          // Show the selected menu
          menu.style.display = 'block';
          menu.style.left = (e as MouseEvent).pageX + 'px';
          menu.style.top = (e as MouseEvent).pageY + 'px';
          menu.setAttribute('callerId', triggerId);
        });
      });
    
      // Hide menus when clicking outside
      document.addEventListener('click', function ()
      {
        menus.forEach(menu => menu.style.display = 'none');
      });
    });
}

export function AddContextMenuHandler(trigger: HTMLElement, dataMenu: string, triggerId: string)
{
  const menus = document.getElementById(dataMenu)!;

  trigger.addEventListener('contextmenu', function (e)
  {
      e.preventDefault();
      e.stopPropagation();
      const menuId = trigger.getAttribute('data-menu')!;
      const menu = document.getElementById(menuId)! as HTMLElement;

      // Hide all menus
      menus.style.display = 'none';

      // Show the selected menu
      menu.style.display = 'block';
      menu.style.left = (e as MouseEvent).pageX + 'px';
      menu.style.top = (e as MouseEvent).pageY + 'px';
      menu.setAttribute('callerId', triggerId);
  });

  if (!trackedMenus.includes(dataMenu))
  {
    document.addEventListener('mouseup', function () { menus.style.display = 'none'; });
    trackedMenus.push(dataMenu);
  }
  // Hide menus when clicking outside
  
}