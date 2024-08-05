export function AddContextMenuHandlers(triggers: string, dataMenus: string)
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
        });
      });
    
      // Hide menus when clicking outside
      document.addEventListener('click', function ()
      {
        menus.forEach(menu => menu.style.display = 'none');
      });
    });
}

export function AddContextMenuHandler(trigger: HTMLElement, dataMenu: string)
{
  const menus = document.querySelectorAll<HTMLElement>(dataMenu)!;

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
  });

  // Hide menus when clicking outside
  document.addEventListener('click', function ()
  {
    menus.forEach(menu => menu.style.display = 'none');
  });
}