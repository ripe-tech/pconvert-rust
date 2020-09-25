const js = import("./node_modules/pconvert_rust/pconvert_rust.js");

const apiFunctionSelect = document.querySelector("div#api select");
const blendButton = document.querySelector("button#blend");
const benchmarkButton = document.querySelector("button#benchmark");
const canvas = document.querySelector("canvas#composition");
const inputFiles = document.querySelector("input#files");
const inputAlgorithm = document.querySelector("input#algorithm");
const metadata = document.querySelector("pre#metadata");
const textareaAlgorithms = document.querySelector("textarea#algorithms");

blendButton.addEventListener("click", () => blend());
benchmarkButton.addEventListener("click", () => benchmark());

const API_FUNCTIONS = {
  blend_images_data: "blend_images_data",
  blend_images: "blend_images",
  blend_multiple_data: "blend_multiple_data",
  blend_multiple: "blend_multiple"
};

async function blend() {
  const pconvert = await js.then(js => js);
  const apiFunction = apiFunctionSelect.options[apiFunctionSelect.selectedIndex].value;

  let composition;
  switch (apiFunction) {
    case API_FUNCTIONS.blend_images_data: {
      const top = getImageData(await loadImage(inputFiles.files[0]));
      const bot = getImageData(await loadImage(inputFiles.files[1]));
      const algorithm = inputAlgorithm.value;
      composition = pconvert.blend_images_data(top, bot, algorithm == "" ? null : algorithm);
      break;
    }

    case API_FUNCTIONS.blend_images: {
      const top = inputFiles.files[0];
      const bot = inputFiles.files[1];
      const algorithm = inputAlgorithm.value;
      const file = await pconvert.blend_images(top, bot, algorithm == "" ? null : algorithm);
      composition = getImageData(await loadImage(file));
      break;
    }

    case API_FUNCTIONS.blend_multiple_data: {
      const data = [];
      for (file of inputFiles.files) {
        const imageData = getImageData(await loadImage(file));
        data.push(imageData);
      }

      const algorithms = textareaAlgorithms.value;
      if (isJSONParsable(algorithms)) {
        const algorithmsJSON = JSON.parse(algorithms)["algorithms"];
        composition = await pconvert.blend_multiple_data(data, null, algorithmsJSON);
      }
      else {
        const algorithm = inputAlgorithm.value;
        composition = await pconvert.blend_multiple_data(data, algorithm == "" ? null : algorithm);
      }
      break;
    }

    case API_FUNCTIONS.blend_multiple: {
      const algorithms = textareaAlgorithms.value;
      if (isJSONParsable(algorithms)) {
        const algorithmsJSON = JSON.parse(algorithms)["algorithms"];
        const file = await pconvert.blend_multiple(inputFiles.files, null, algorithmsJSON);
        composition = getImageData(await loadImage(file));
      }
      else {
        const algorithm = inputAlgorithm.value;
        const file = await pconvert.blend_multiple(inputFiles.files, algorithm == "" ? null : algorithm);
        composition = getImageData(await loadImage(file));
      }
      break;
    }

    default:
      console.log("Invalid API function");
  }

  drawComposition(composition);
}

async function benchmark() {
  const pconvert = await js.then(js => js);
  const apiFunction = apiFunctionSelect.options[apiFunctionSelect.selectedIndex].value;

  let composition;
  switch (apiFunction) {
    case API_FUNCTIONS.blend_images_data:
    case API_FUNCTIONS.blend_images:
      {
        const top = inputFiles.files[0];
        const bot = inputFiles.files[1];
        const algorithm = inputAlgorithm.value;
        const file = await pconvert.blend_images_benchmark(top, bot, algorithm == "" ? null : algorithm);
        composition = getImageData(await loadImage(file));
        break;
      }

    case API_FUNCTIONS.blend_multiple_data:
    case API_FUNCTIONS.blend_multiple:
      {
        const algorithms = textareaAlgorithms.value;
        if (isJSONParsable(algorithms)) {
          const algorithmsJSON = JSON.parse(algorithms)["algorithms"];
          const file = await pconvert.blend_multiple_benchmark(inputFiles.files, null, algorithmsJSON);
          composition = getImageData(await loadImage(file));
        }
        else {
          const algorithm = inputAlgorithm.value;
          const file = await pconvert.blend_multiple_benchmark(inputFiles.files, algorithm == "" ? null : algorithm);
          composition = getImageData(await loadImage(file));
        }
        break;
      }

    default:
      console.log("Invalid API function");
  }
  drawComposition(composition);
}

async function getPConvertMetadata() {
  const pconvert = await js.then(js => js);
  const constants = pconvert.get_module_constants();
  metadata.innerHTML = JSON.stringify(constants, undefined, 2);
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

function isJSONParsable(str) {
  try {
    JSON.parse(str);
  } catch (e) {
    return false;
  }
  return true;
}

getPConvertMetadata()
