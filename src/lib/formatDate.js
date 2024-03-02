export function formatDate(dateString) {
    const options = {
        year: 'numeric',
        month: 'numeric',
        day: 'numeric'
    }; 
    const date = new Date(dateString);
    return date.toLocaleDateString(undefined, options);
}