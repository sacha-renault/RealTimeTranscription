export function formatDate(date: Date) {
    const now = new Date();

    // Check if the date is today
    const isToday = now.toDateString() === date.toDateString();

    // Check if the date is yesterday
    const yesterday = new Date();
    yesterday.setDate(yesterday.getDate() - 1);
    const isYesterday = yesterday.toDateString() === date.toDateString();

    if (isToday) {
        return `Today at ${date.toLocaleTimeString(undefined)}`;
    } else if (isYesterday) {
        return `Yesterday at ${date.toLocaleTimeString(undefined)}`;
    } else {
        // For other dates, use dd-mm-yy hh:mm
        const day = String(date.getDate()).padStart(2, '0');
        const month = String(date.getMonth() + 1).padStart(2, '0');
        const year = String(date.getFullYear()).slice(-2);
        const time = date.toLocaleTimeString(undefined);

        return `${day}-${month}-${year} ${time}`;
    }
}
