export function formatDate(dateString) {
    const options = {
        year: 'numeric',
        month: 'numeric',
        day: 'numeric'
    }; 
    const date = new Date(dateString);
    return date.toLocaleDateString(undefined, options);
}

export function formatDateYYYYMMDD(date) {
    const year = date.getFullYear(); 
    const month = (date.getMonth() + 1).toString().padStart(2,'0'); 
    const day = date.getDate().toString().padStart(2,'0'); 

    return `${year}-${month}-${day}`; 
}