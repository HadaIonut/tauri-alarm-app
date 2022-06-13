//format example: y-M-d h:m:s
import {readTextFile, writeFile} from "@tauri-apps/api/fs";
import {alarms} from "../store/alarms.js";
import {dataDir} from "@tauri-apps/api/path";
import {invoke} from "@tauri-apps/api/tauri";

export const formatDate = (date, format) => {
  const dateMap = {
    's': 'sec',
    'm': 'min',
    'h': 'hour',
    'd': 'day',
    'M': 'month',
    'y': 'year'
  }

  const dateValues = {
    sec: String(date.getSeconds()).padStart(2, '0'),
    min: String(date.getMinutes()).padStart(2, '0'),
    hour: String(date.getHours()).padStart(2, '0'),
    day: String(date.getDate()).padStart(2, '0'),
    month: String(date.getMonth() + 1).padStart(2, '0'),
    year: date.getFullYear()
  }
  return [...format].map((val) => {
    let dateField = dateMap[val] ?? val;
    return dateValues[dateField] ?? val;
  }).join('');
}

export const addMinutes = (date, minutes) => {
  return new Date(date.getTime() + minutes * 60000);
}

export const addHours = (date, minutes) => {
  return new Date(date.getTime() + minutes * 60000 * 60);
}

export const updateSaveFile = async (data) => {
  await writeFile(
    {
      contents: JSON.stringify(data),
      path: `${await dataDir()}/data.json`,
    },
  )
}

export const readSaveFile = async () => {
  let data = JSON.parse(await readTextFile(`${await dataDir()}/data.json`));

  if (Array.isArray(data)) {
    await invoke('init_file_save', {alarms: data})
    //TODO: also check if the alarm has expired and fire an alarm

    return data;
  } else {
    return [];
  }

}