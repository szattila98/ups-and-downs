import dayjs, { Dayjs } from 'dayjs';

export const DATE_FORMAT = 'YYYY-MM-DD';

export const DATETIME_FORMAT = `${DATE_FORMAT} HH:mm:ss`;

export const getDate = (date: Dayjs) => date.format(DATETIME_FORMAT);

export const getToday = () => getDate(dayjs());
