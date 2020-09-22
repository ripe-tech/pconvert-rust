const js = import("./node_modules/pconvert_rust/pconvert_rust.js");

const canvas = document.querySelector("canvas#composition");
const input = document.querySelector("input#files");
const apiFunctionSelect = document.querySelector("div#api select");

input.addEventListener("change", () => execute());

const API_FUNCTIONS = {
  blend_images_data: "blend_images_data",
  blend_images: "blend_images",
  blend_multiple_data: "blend_multiple_data"
};

async function execute() {
  const pconvert = await js.then(js => js);
  const apiFunction = apiFunctionSelect.options[apiFunctionSelect.selectedIndex].value;  
  switch(apiFunction) {
    case API_FUNCTIONS.blend_images_data: {
      const top = getImageData(await loadImage(input.files[0]));
      const bot = getImageData(await loadImage(input.files[1]));
      const composition = pconvert.blend_images_data(top, bot)
      drawComposition(composition);
      break;
    }
    case API_FUNCTIONS.blend_images: {
      const top = input.files[0];
      const bot = input.files[1];
      const composition = await pconvert.blend_images(top, bot);
      drawComposition(composition);
      break;
    }
    case API_FUNCTIONS.blend_multiple_data: {
      const data = [];
      for(file of input.files){
        const imageData = getImageData(await loadImage(file));
        data.push(imageData);
      }
      let algorithms = ["alpha", "multiplicative", {
        algorithm: "mask_top",
        params: {
          factor: 0.8,
          tobias: true,
          alberto: "caeiro",
          integer: 3,
        }
      }];
      const composition = pconvert.blend_multiple_data(data, null, algorithms);
      drawComposition(composition);
      break;
    }
    default:
      console.log("Invalid API function");
  }
}

function loadImage(url) {
  return new Promise((resolve, reject) => {
    const img = new Image();
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
  context.drawImage(img, 0, 0);
  return context.getImageData(0, 0, img.width, img.height);
}

function drawComposition(composition) {
  canvas.width = composition.width;
  canvas.height = composition.height;
  const context = canvas.getContext("2d");
  context.putImageData(composition, 0, 0);
}
