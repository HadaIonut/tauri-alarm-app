//format example: y-M-d h:m:s
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