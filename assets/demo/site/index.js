const js = import("./node_modules/pconvert_rust/pconvert_rust.js");

const canvas = document.querySelector("canvas#composition");
const input = document.querySelector("input#files");

input.addEventListener("change", () => execute())

async function execute() {
  let top = getImageData(await loadImage(input.files[0]));
  let bot = getImageData(await loadImage(input.files[1]));

  js.then(js => {
    let composition = js.blend_images(top, bot, "multiplicative", false)
    canvas.width = composition.width;
    canvas.height = composition.height;
    const context = canvas.getContext("2d");
    context.putImageData(composition, 0, 0);
  }); 
}

async function loadImage(url) {
  return new Promise((resolve, reject) => {
    let img = new Image();
    img.addEventListener('load', e => resolve(img));
    img.addEventListener('error', () => {
      reject(new Error(`Failed to load image's URL: ${url}`));
    });
    img.src = URL.createObjectURL(url);
  });
}

function getImageData(img) {
  const canvas = document.createElement('canvas');
  const context = canvas.getContext('2d');
  canvas.width = img.width;
  canvas.height = img.height;
  context.drawImage(img, 0, 0 );
  return context.getImageData(0, 0, img.width, img.height);
}
