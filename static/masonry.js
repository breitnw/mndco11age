// This code is adapted from Andy Barefoot's guide. Thank you!
// https://medium.com/@andybarefoot/a-masonry-style-layout-using-css-grid-8c663d355ebb

function resizeCard(item){
  grid = document.getElementsByClassName("masonry")[0];
  grid_style = window.getComputedStyle(grid);
  rowHeight = parseInt(grid_style.getPropertyValue('grid-auto-rows'));
  rowGap = parseInt(grid_style.getPropertyValue('grid-row-gap'));
  rowSpan = Math.ceil((item.querySelector(".card-contents").getBoundingClientRect().height+rowGap)/(rowHeight+rowGap));
    item.style.gridRowEnd = "span "+rowSpan;
}

function resizeAllCards(){
  allItems = document.getElementsByClassName("card");
  for(x=0;x<allItems.length;x++){
    resizeCard(allItems[x]);
  }
}

function loadInstance(instance){
	item = instance.elements[0];
  item.classList.add("loaded")

  // every time a new instance is loaded, resize all cards to fit with the layout
	resizeAllCards();
}

// resize cards when the window size changes
window.addEventListener("resize", resizeAllCards);

// resize incrementally on load
allItems = document.getElementsByClassName("card");
for(x=0;x<allItems.length;x++){
  const item = allItems[x]
  const image = item.querySelector("img")
  const loadingAnimation = item.querySelector(".loading-animation")

  imagesLoaded(allItems[x], loadInstance);

  // if (image.decode) {
  //   image.decode()
  //     .then(() => {
  //       console.log('GIF ready with dimensions', item.naturalWidth, item.naturalHeight);
  //       resizeAllCards()
  //       // resizeCard(item)
  //     })
  // }
}

// finally, resize now
resizeAllCards();
