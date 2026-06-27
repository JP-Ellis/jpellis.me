const KILO = 1000;
const MS_PER_SEC = 1000;
const MINS_PER_HOUR = 60;
const SECS_PER_HOUR = 3600;
const HOURS_PER_DAY = 24;
const SECS_PER_DAY = 86_400;
const DAYS_PER_WEEK = 7;
const WEEKS_BEFORE_MONTHS = 5;
const DAYS_PER_MONTH = 30;
const MONTHS_PER_YEAR = 12;
const DAYS_PER_YEAR = 365;
const MIN_DATE_PARTS = 3;
const JUST_NOW_THRESHOLD_MINS = 1;
const STRIP_LEADING_ZEROS = /^0+/u;

const MONTHS: Record<string, string> = {
  "01": "Jan",
  "02": "Feb",
  "03": "Mar",
  "04": "Apr",
  "05": "May",
  "06": "Jun",
  "07": "Jul",
  "08": "Aug",
  "09": "Sep",
  "10": "Oct",
  "11": "Nov",
  "12": "Dec",
};

export function formatStars(n: number): string {
  if (n >= KILO) {
    return `${(n / KILO).toFixed(1)}k`;
  }
  return String(n);
}

export function formatShortDate(iso: string): string {
  const [datePart] = iso.split("T");
  const parts = datePart.split("-");
  if (parts.length < MIN_DATE_PARTS) {
    return iso;
  }
  const month = MONTHS[parts[1]] ?? parts[1];
  const day = parts[2].replace(STRIP_LEADING_ZEROS, "") || "1";
  return `${day} ${month} ${parts[0]}`;
}

export function formatRelativeDate(iso: string, now: Date): string {
  const dt = new Date(iso);
  if (Number.isNaN(dt.getTime())) {
    return iso;
  }
  const secs = Math.max(0, (now.getTime() - dt.getTime()) / MS_PER_SEC);
  if (secs < SECS_PER_HOUR) {
    const mins = Math.floor(secs / MINS_PER_HOUR);
    if (mins <= JUST_NOW_THRESHOLD_MINS) {
      return "just now";
    }
    return `${mins}m ago`;
  }
  const hours = Math.floor(secs / SECS_PER_HOUR);
  if (hours < HOURS_PER_DAY) {
    return `${hours}h ago`;
  }
  const days = Math.floor(secs / SECS_PER_DAY);
  if (days < DAYS_PER_WEEK) {
    return `${days}d ago`;
  }
  const weeks = Math.floor(days / DAYS_PER_WEEK);
  if (weeks < WEEKS_BEFORE_MONTHS) {
    return `${weeks}w ago`;
  }
  const months = Math.floor(days / DAYS_PER_MONTH);
  if (months < MONTHS_PER_YEAR) {
    return `${months}mo ago`;
  }
  return `${Math.floor(days / DAYS_PER_YEAR)}y ago`;
}
