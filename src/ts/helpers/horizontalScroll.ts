export function makeHorizontalScroll(elementId) {
    var item = document.getElementById(elementId);

    if (item == null) return;

    item!.addEventListener("wheel", function (e) {
        if (e.deltaY > 0) item!.scrollLeft += 100;
        else item!.scrollLeft -= 100;
    });
}