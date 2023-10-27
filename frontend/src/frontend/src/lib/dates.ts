export function formatUploadDate(uploaded_at: BigInt) {
  const dateOptions: Intl.DateTimeFormatOptions = {
    weekday: "long",
    year: "numeric",
    month: "long",
    day: "numeric",
    timeZone: "CET",
    hour12: false,
  };
  let uploadedAt = new Date(Math.floor(Number(uploaded_at) / 1000000));
  return uploadedAt.toLocaleTimeString("en-CH", dateOptions);
}
