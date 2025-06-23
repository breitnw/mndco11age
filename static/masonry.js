// This code is adapted from Andy Barefoot's guide. Thank you!
// https://medium.com/@andybarefoot/a-masonry-style-layout-using-css-grid-8c663d355ebb

function resizeGridItem(item){
  grid = document.getElementsByClassName("masonry")[0];
  rowHeight = parseInt(window.getComputedStyle(grid).getPropertyValue('grid-auto-rows'));
  rowGap = parseInt(window.getComputedStyle(grid).getPropertyValue('grid-row-gap'));
  console.log(rowGap)
  rowSpan = Math.ceil((item.querySelector(".contents").getBoundingClientRect().height+rowGap)/(rowHeight+rowGap));
    item.style.gridRowEnd = "span "+rowSpan;
}

function resizeAllGridItems(){
  allItems = document.getElementsByClassName("card");
  for(x=0;x<allItems.length;x++){
    resizeGridItem(allItems[x]);
  }
}

function resizeInstance(instance){
	item = instance.elements[0];
  resizeGridItem(item);
}

window.addEventListener("resize", resizeAllGridItems);
window.addEventListener("load", resizeAllGridItems);

// also resize incrementally on load
allItems = document.getElementsByClassName("card");
for(x=0;x<allItems.length;x++){
  imagesLoaded(allItems[x], resizeInstance);
}
