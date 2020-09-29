const js = import("./node_modules/pconvert_rust/pconvert_rust.js");
const placeholderJSON = import("./example_algorithms.json");

const apiFunctionSelect = document.querySelector("div#api select");
const blendButton = document.querySelector("button#blend");
const benchmarkButton = document.querySelector("button#benchmark");
const canvas = document.querySelector("canvas#composition");
const inputFiles = document.querySelector("input#files");
const selectAlgorithm = document.querySelector("select#algorithm");
const metadata = document.querySelector("pre#metadata");
const textareaAlgorithms = document.querySelector("textarea#algorithms");

blendButton.addEventListener("click", () => blend());
benchmarkButton.addEventListener("click", () => benchmark());

setPConvertMetadata()
setAlgorithmsPlaceholder()
setAlgorithmSelectOptions()

const API_FUNCTIONS = {
  blendImagesData: "blendImagesData",
  blendImages: "blendImages",
  blendMultipleData: "blendMultipleData",
  blendMultiple: "blendMultiple"
};

async function blend() {
  const pconvert = await js.then(js => js);
  const apiFunction = apiFunctionSelect.options[apiFunctionSelect.selectedIndex].value;

  let composition;
  switch (apiFunction) {
    case API_FUNCTIONS.blendImagesData: {
      const top = getImageData(await loadImage(inputFiles.files[0]));
      const bot = getImageData(await loadImage(inputFiles.files[1]));
      const algorithm = selectAlgorithm.value;
      composition = pconvert.blendImagesData(top, bot, algorithm == "" ? null : algorithm);
      break;
    }

    case API_FUNCTIONS.blendImages: {
      const top = inputFiles.files[0];
      const bot = inputFiles.files[1];
      const algorithm = selectAlgorithm.value;
      const file = await pconvert.blendImages(top, bot, "result", algorithm == "" ? null : algorithm);
      composition = getImageData(await loadImage(file));
      break;
    }

    case API_FUNCTIONS.blendMultipleData: {
      const data = [];
      for (file of inputFiles.files) {
        const imageData = getImageData(await loadImage(file));
        data.push(imageData);
      }

      const algorithms = textareaAlgorithms.value;
      if (isJSONParsable(algorithms)) {
        const algorithmsJSON = JSON.parse(algorithms)["algorithms"];
        composition = await pconvert.blendMultipleData(data, null, algorithmsJSON);
      }
      else {
        const algorithm = selectAlgorithm.value;
        composition = await pconvert.blendMultipleData(data, algorithm == "" ? null : algorithm);
      }
      break;
    }

    case API_FUNCTIONS.blendMultiple: {
      const algorithms = textareaAlgorithms.value;
      if (isJSONParsable(algorithms)) {
        const algorithmsJSON = JSON.parse(algorithms)["algorithms"];
        const file = await pconvert.blendMultiple(inputFiles.files, "result", null, algorithmsJSON);
        composition = getImageData(await loadImage(file));
      }
      else {
        const algorithm = selectAlgorithm.value;
        const file = await pconvert.blendMultiple(inputFiles.files, "result", algorithm == "" ? null : algorithm);
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
  switch (apiFunction) {
    case API_FUNCTIONS.blendImagesData:
    case API_FUNCTIONS.blendImages:
      {
        const top = inputFiles.files[0];
        const bot = inputFiles.files[1];
        await pconvert.blendImagesBenchmarkAll(top, bot);
        break;
      }

    case API_FUNCTIONS.blendMultipleData:
    case API_FUNCTIONS.blendMultiple:
      {
        const algorithms = textareaAlgorithms.value;
        if (isJSONParsable(algorithms)) {
          const algorithmsJSON = JSON.parse(algorithms)["algorithms"];
          await pconvert.blendMultipleBenchmark(inputFiles.files, null, algorithmsJSON);
        }
        else {
          await pconvert.blendMultipleBenchmarkAll(inputFiles.files);
        }
        break;
      }

    default:
      console.log("Invalid API function");
  }
}

async function setPConvertMetadata() {
  const pconvert = await js.then(js => js);
  const constants = pconvert.getModuleConstants();
  metadata.innerHTML = JSON.stringify(constants, undefined, 2);
}

async function setAlgorithmsPlaceholder() {
  const json = await placeholderJSON.then(json => json);
  const placeholder = JSON.stringify(json.default, undefined, 2)
  textareaAlgorithms.setAttribute("placeholder", placeholder)
}

async function setAlgorithmSelectOptions() {
  const pconvert = await js.then(js => js);
  const options = pconvert.getModuleConstants().ALGORITHMS;

  for(option of options) {
    const optionEl = document.createElement("option");
    optionEl.text = option;
    optionEl.value = option;
    selectAlgorithm.appendChild(optionEl);
  }
}

function loadImage(file) {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.addEventListener('load', e => resolve(img));
    img.addEventListener('error', (e) => {
      reject(e);
    });
    img.src = URL.createObjectURL(file);
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
